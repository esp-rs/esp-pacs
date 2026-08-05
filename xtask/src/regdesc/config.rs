use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use super::model::{
    ChipInfo, Peripheral, PeripheralInstance, PeripheralInterrupt, RegdescFragment,
};

#[derive(Debug, Deserialize)]
struct PeripheralsFile {
    peripheral_instances: Vec<PeripheralInstanceRaw>,
}

#[derive(Debug, Deserialize)]
struct PeripheralInstanceRaw {
    name: String,
    peripheral: String,
    base_addr: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InterruptsFile {
    peripheral_interrupts: Vec<PeripheralInterruptRaw>,
}

#[derive(Debug, Deserialize)]
struct PeripheralInterruptRaw {
    name: String,
    instance: String,
    value: u32,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DescriptionsFile {
    peripheral_descriptions: Vec<PeripheralDescription>,
}

#[derive(Debug, Deserialize)]
struct PeripheralDescription {
    peripheral: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct CsvEntryOptions {
    pub name: Option<String>,
    /// Warning names to ignore (reserved for future use; present in YAML for compatibility).
    #[serde(default)]
    #[allow(dead_code)]
    pub wno: Vec<String>,
}

/// Reads `regdesc.yml` — which CSV files to parse and any name overrides.
pub fn load_regdesc_config(path: &Path) -> Result<HashMap<String, CsvEntryOptions>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("reading regdesc config {}", path.display()))?;
    serde_yaml::from_str(&content)
        .with_context(|| format!("parsing regdesc config {}", path.display()))
}

/// Loads `peripherals.yml` and `interrupts.yml` (base addresses, instances, IRQ numbers).
pub fn load_metadata(
    config_dir: &Path,
) -> Result<(Vec<PeripheralInstance>, Vec<PeripheralInterrupt>)> {
    let peripherals_path = config_dir.join("peripherals.yml");
    let interrupts_path = config_dir.join("interrupts.yml");

    let peripherals_content = fs::read_to_string(&peripherals_path)
        .with_context(|| format!("reading {}", peripherals_path.display()))?;
    let interrupts_content = fs::read_to_string(&interrupts_path)
        .with_context(|| format!("reading {}", interrupts_path.display()))?;

    let peripherals: PeripheralsFile = serde_yaml::from_str(&peripherals_content)
        .with_context(|| format!("parsing {}", peripherals_path.display()))?;
    let interrupts: InterruptsFile = serde_yaml::from_str(&interrupts_content)
        .with_context(|| format!("parsing {}", interrupts_path.display()))?;

    let instances = peripherals
        .peripheral_instances
        .into_iter()
        .map(|raw| PeripheralInstance {
            name: raw.name,
            peripheral: raw.peripheral,
            base_addr: parse_addr(&raw.base_addr),
            description: raw.description,
        })
        .collect();

    let peripheral_interrupts = interrupts
        .peripheral_interrupts
        .into_iter()
        .map(|raw| PeripheralInterrupt {
            name: raw.name,
            instance: raw.instance,
            value: raw.value,
            description: raw.description,
        })
        .collect();

    Ok((instances, peripheral_interrupts))
}

/// Loads the shared peripheral description blurbs from YAML.
pub fn load_descriptions(path: &Path) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    let file: DescriptionsFile = serde_yaml::from_str(&content)
        .with_context(|| format!("parsing {}", path.display()))?;
    Ok(file
        .peripheral_descriptions
        .into_iter()
        .map(|entry| (entry.peripheral, entry.description))
        .collect())
}

/// Fills in each instance's description, or falls back to a generic one.
pub fn apply_instance_descriptions(
    instances: &mut [PeripheralInstance],
    descriptions: &HashMap<String, String>,
) {
    for instance in instances.iter_mut() {
        instance.description = descriptions
            .get(&instance.name)
            .cloned()
            .or_else(|| {
                log::warn!("{} has no description", instance.name);
                Some(format!("{} Peripheral", instance.name))
            });
    }
}

/// Bundles everything into one struct that's ready for SVD writing.
pub fn build_fragment(
    chip: &str,
    peripherals: Vec<Peripheral>,
    instances: Vec<PeripheralInstance>,
    interrupts: Vec<PeripheralInterrupt>,
) -> RegdescFragment {
    RegdescFragment {
        peripherals,
        peripheral_instances: instances,
        peripheral_interrupts: interrupts,
        chip_info: ChipInfo::from_chip_id(chip),
    }
}

/// Parses `"0x20343000"`-style addresses into a `u32`.
fn parse_addr(value: &str) -> u32 {
    let value = value.trim();
    let parsed = if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16)
    } else {
        value.parse()
    };
    match parsed {
        Ok(addr) => addr,
        Err(_) => {
            log::warn!("invalid peripheral base address '{value}', using 0");
            0
        }
    }
}

/// Path to `xtask/regdesc/`.
pub fn regdesc_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("regdesc")
}

/// Path to `xtask/regdesc/chips/{chip}/`.
pub fn chip_config_dir(chip: &str) -> PathBuf {
    regdesc_root().join("chips").join(chip)
}

/// Path to the shared `peripheral_descriptions.yml`.
pub fn peripheral_descriptions_path() -> PathBuf {
    regdesc_root().join("peripheral_descriptions.yml")
}

/// Default SVD output: `target/generated_svds/{chip}.svd`.
pub fn default_output_path(workspace: &Path, chip: &str) -> PathBuf {
    workspace
        .join("target")
        .join("generated_svds")
        .join(format!("{chip}.svd"))
}
