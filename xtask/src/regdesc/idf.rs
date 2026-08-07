//! Parse Espressif IDF headers used as SVD metadata sources.

use std::{fs, path::Path, sync::LazyLock};

use anyhow::{bail, Context, Result};
use regex::Regex;

use super::model::{PeripheralInstance, PeripheralInterrupt};

static DR_REG_BASE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^#define\s+DR_REG_([A-Z0-9_]+)_BASE\s+(0x[0-9a-fA-F]+)"#).unwrap()
});
static IEEE802154_BASE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?m)^#define\s+IEEE802154_REG_BASE\s+(0x[0-9a-fA-F]+)"#).unwrap()
});
static ETS_ENUM_ENTRY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*(ETS_[A-Z0-9_]+)\s*(?:=\s*([^,/]+))?\s*,").unwrap());

/// GDVS CSV filename → peripheral type when probing would not match.
pub fn csv_type_overrides() -> &'static [(&'static str, &'static str)] {
    &[
        ("apb_saradc_reg.csv", "APB_SARADC"),
        ("ecc_mult_reg.csv", "ECC"),
        ("efuse_mem_reg.csv", "EFUSE"),
        ("gpio_ext_reg.csv", "GPIOSD"),
        ("spi_mem_reg.csv", "SPI0"),
        ("spi1_mem_reg.csv", "SPI1"),
        ("spi2_reg.csv", "SPI2"),
        ("spi3_reg.csv", "SPI3"),
        ("usb_serial_jtag_reg.csv", "USB_DEVICE"),
        ("usb_device_reg.csv", "USB_DEVICE"),
        ("usb_otgfs_core_ctrl_reg.csv", "USB_OTGFS"),
    ]
}

/// IDF `DR_REG_<STEM>_BASE` stem → (instance name, peripheral type).
fn reg_base_alias(stem: &str) -> Option<(&'static str, &'static str)> {
    Some(match stem {
        "SPIMEM0" => ("SPI0", "SPI0"),
        "SPIMEM1" => ("SPI1", "SPI1"),
        "GPSPI2" => ("SPI2", "SPI2"),
        "GPSPI3" => ("SPI3", "SPI3"),
        "GDMA" => ("DMA", "DMA"),
        "AHB_DMA" => ("DMA", "AHB_DMA"),
        "USB_SERIAL_JTAG" => ("USB_DEVICE", "USB_DEVICE"),
        "INTMTX" | "INTMTX0" => ("INTERRUPT_CORE0", "INTERRUPT_CORE0"),
        "INTMTX1" => ("INTERRUPT_CORE1", "INTERRUPT_CORE1"),
        "TIMERG0" => ("TIMG0", "TIMG"),
        "TIMERG1" => ("TIMG1", "TIMG"),
        "GPIO_EXT" => ("GPIO_SD", "GPIOSD"),
        "HP_SYSTEM" => ("HP_SYS", "HP_SYSTEM"),
        "LP_ANA_PERI" => ("LP_ANA", "LP_ANA"),
        "MCPWM" | "MCPWM0" => ("MCPWM0", "MCPWM"),
        "MCPWM1" => ("MCPWM1", "MCPWM"),
        "TWAI" => ("TWAI0", "TWAI"),
        "TWAIFD" => ("TWAI0", "TWAIFD"),
        "I2S0" => ("I2S0", "I2S"),
        "UHCI0" => ("UHCI0", "UHCI"),
        "UART0" => ("UART0", "UART"),
        "UART1" => ("UART1", "UART"),
        "I2C0" => ("I2C0", "I2C"),
        "I2C1" => ("I2C1", "I2C"),
        "LPPERI" => ("LP_PERI", "LPPERI"),
        "SAMPLE_RATE_CONVERTER" => ("ASRC", "ASRC"),
        "TRNG" => ("RNG", "RNG"),
        "TOUCH_SENS" => ("TOUCH", "TOUCH"),
        "USB_OTG_FS_CORE0" => ("USB0", "USB_OTGFS"),
        "USB_OTG_FS_CORE1" => ("USB1", "USB_OTGFS"),
        "HP_MEM_APM" => ("HP_MEM_APM", "HP_MEM_APM"),
        _ => return None,
    })
}

/// Reads `soc/interrupts.h` and returns a flat IRQ list (name + hardware source
/// id).
///
/// Names are normalized toward historical esp-pacs interrupt identifiers.
pub fn load_interrupts_h(path: &Path, chip: &str) -> Result<Vec<PeripheralInterrupt>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("reading interrupts header {}", path.display()))?;
    parse_interrupts_h(&content, chip)
        .with_context(|| format!("parsing interrupts header {}", path.display()))
}

/// Reads `register/soc/reg_base.h` into peripheral instances.
pub fn load_reg_base_h(path: &Path) -> Result<Vec<PeripheralInstance>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("reading reg_base header {}", path.display()))?;
    parse_reg_base_h(&content)
        .with_context(|| format!("parsing reg_base header {}", path.display()))
}

pub fn parse_interrupts_h(content: &str, chip: &str) -> Result<Vec<PeripheralInterrupt>> {
    let mut next_value: i64 = 0;
    let mut out = Vec::new();
    let mut in_enum = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("typedef enum") {
            in_enum = true;
            next_value = 0;
            continue;
        }
        if !in_enum {
            continue;
        }
        if trimmed.starts_with('}') {
            break;
        }

        let Some(caps) = ETS_ENUM_ENTRY.captures(line) else {
            continue;
        };
        let ets = &caps[1];
        if ets == "ETS_MAX_INTR_SOURCE" {
            break;
        }
        if let Some(explicit) = caps.get(2) {
            let expr = explicit.as_str().trim();
            // Skip symbolic aliases like `= ETS_FOO_INTR_SOURCE`; keep counting.
            if let Ok(v) = parse_c_int(expr) {
                next_value = v;
            }
        }
        if next_value < 0 || next_value > u32::MAX as i64 {
            bail!("interrupt value out of range for {ets}: {next_value}");
        }
        out.push(PeripheralInterrupt {
            name: pacify_irq_name(chip, &irq_name_from_ets(ets)),
            value: next_value as u32,
            description: None,
        });
        next_value += 1;
    }

    if out.is_empty() {
        bail!("no ETS_* interrupt sources found");
    }
    Ok(out)
}

pub fn parse_reg_base_h(content: &str) -> Result<Vec<PeripheralInstance>> {
    let mut out = Vec::new();

    for caps in DR_REG_BASE.captures_iter(content) {
        let stem = &caps[1];
        let addr = u32::from_str_radix(caps[2].trim_start_matches("0x"), 16)
            .with_context(|| format!("invalid base address for DR_REG_{stem}_BASE"))?;
        let (name, peripheral) = match reg_base_alias(stem) {
            Some((n, p)) => (n.to_owned(), p.to_owned()),
            None => (stem.to_owned(), stem.to_owned()),
        };
        out.push(PeripheralInstance {
            name,
            peripheral,
            base_addr: addr,
            description: None,
        });
    }

    if let Some(caps) = IEEE802154_BASE.captures(content) {
        let addr = u32::from_str_radix(caps[1].trim_start_matches("0x"), 16)
            .context("invalid IEEE802154_REG_BASE")?;
        out.push(PeripheralInstance {
            name: "IEEE802154".to_owned(),
            peripheral: "IEEE802154".to_owned(),
            base_addr: addr,
            description: None,
        });
    }

    if out.is_empty() {
        bail!("no DR_REG_*_BASE definitions found");
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn irq_name_from_ets(ets: &str) -> String {
    let mut name = ets.strip_prefix("ETS_").unwrap_or(ets).to_owned();
    if let Some(rest) = name.strip_suffix("_INTR_SOURCE") {
        name = rest.to_owned();
    } else if let Some(rest) = name.strip_suffix("_SOURCE") {
        // e.g. ETS_BT_BB_NMI_SOURCE → BT_BB_NMI
        name = rest.to_owned();
    }
    name
}

/// Map IDF enum stems onto names historically used in esp-pacs PACs.
fn pacify_irq_name(chip: &str, name: &str) -> String {
    // Cross-chip exact renames.
    let name = match name {
        "USB_SERIAL_JTAG" => return "USB_DEVICE".to_owned(),
        "GPIO_INTERRUPT_PRO" => return "GPIO".to_owned(),
        "GPIO_INTERRUPT_PRO_NMI" => return "GPIO_NMI".to_owned(),
        "PWM" | "PWM0" => return "MCPWM0".to_owned(),
        "PWM1" => return "MCPWM1".to_owned(),
        "TWAI" => return "TWAI0".to_owned(),
        "WIFI_MAC_NMI" if matches!(chip, "esp32" | "esp32s2" | "esp32s3") => {
            return "WIFI_NMI".to_owned();
        }
        "GSPI2" if chip.contains("c6") => return "SPI2".to_owned(),
        "GSPI2" => return "GPSPI2".to_owned(),
        other => other,
    };

    if let Some(idx) = name.strip_prefix("CPU_INTR_FROM_CPU_") {
        return format!("FROM_CPU_INTR{idx}");
    }

    // Timer IRQs stay as IDF names (`TG0_T0`). Chips that want `TG0_T0_LEVEL`
    // in the PAC do that in svd patches (e.g. esp32p4 / esp32s31).

    name.to_owned()
}

fn parse_c_int(expr: &str) -> Result<i64> {
    let expr = expr.trim();
    if let Some(hex) = expr.strip_prefix("0x").or_else(|| expr.strip_prefix("0X")) {
        return i64::from_str_radix(hex, 16).context("hex literal");
    }
    expr.parse().context("decimal literal")
}
