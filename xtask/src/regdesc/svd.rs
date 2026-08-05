use std::{fmt::Write as _, sync::LazyLock};

use regex::Regex;

use super::{
    model::{
        ChipInfo,
        Peripheral,
        PeripheralInstance,
        PeripheralInterrupt,
        RegdescFragment,
        Register,
        Repeat,
    },
    util::{guess_field_access, simplify_name},
};

static FIELD_DESC_INDEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\$[a-z]").unwrap());

/// Turns a chip model into a full SVD XML string.
pub fn write_svd(fragment: &RegdescFragment, version: u32) -> String {
    let mut out = String::new();
    let chip = &fragment.chip_info;

    writeln!(out, "<?xml version=\"1.0\" ?>").unwrap();
    writeln!(
        out,
        "<device xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" schemaVersion=\"1.1\" xsi:noNamespaceSchemaLocation=\"CMSIS-SVD_Schema_1.1.xsd\">"
    )
    .unwrap();

    write_device_preamble(&mut out, chip, version);
    write_cpu(&mut out, chip);
    write_peripherals(&mut out, fragment);

    writeln!(out, "</device>").unwrap();
    out
}

/// Writes vendor, chip name, version, description, and the license block.
fn write_device_preamble(out: &mut String, chip: &ChipInfo, version: u32) {
    let year = time_year();
    writeln!(
        out,
        "  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>"
    )
    .unwrap();
    writeln!(out, "  <vendorID>ESPRESSIF</vendorID>").unwrap();
    writeln!(out, "  <name>{}</name>", xml_escape(&chip.name)).unwrap();
    writeln!(out, "  <series>{}</series>", xml_escape(chip.series())).unwrap();
    writeln!(out, "  <version>{version}</version>").unwrap();
    writeln!(
        out,
        "  <description>{}</description>",
        xml_escape(&chip.description)
    )
    .unwrap();
    writeln!(out, "  <licenseText>").unwrap();
    writeln!(
        out,
        "    Copyright {year} Espressif Systems (Shanghai) PTE LTD"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "    Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);"
    )
    .unwrap();
    writeln!(
        out,
        "    you may not use this file except in compliance with the License."
    )
    .unwrap();
    writeln!(out, "    You may obtain a copy of the License at").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        http://www.apache.org/licenses/LICENSE-2.0").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "    Unless required by applicable law or agreed to in writing, software"
    )
    .unwrap();
    writeln!(
        out,
        "    distributed under the License is distributed on an &quot;AS IS&quot; BASIS,"
    )
    .unwrap();
    writeln!(
        out,
        "    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied."
    )
    .unwrap();
    writeln!(
        out,
        "    See the License for the specific language governing permissions and"
    )
    .unwrap();
    writeln!(out, "    limitations under the License.").unwrap();
    writeln!(out, "  </licenseText>").unwrap();
}

/// Writes the `<cpu>` section and global width/reset settings.
fn write_cpu(out: &mut String, chip: &ChipInfo) {
    writeln!(out, "  <cpu>").unwrap();
    writeln!(out, "    <name>{}</name>", chip.cpu_name).unwrap();
    writeln!(out, "    <revision>{}</revision>", chip.cpu_revision).unwrap();
    writeln!(out, "    <endian>{}</endian>", chip.cpu_endian).unwrap();
    writeln!(out, "    <mpuPresent>{}</mpuPresent>", chip.mpu_present).unwrap();
    writeln!(out, "    <fpuPresent>{}</fpuPresent>", chip.fpu_present).unwrap();
    writeln!(
        out,
        "    <nvicPrioBits>{}</nvicPrioBits>",
        chip.nvic_prio_bits
    )
    .unwrap();
    writeln!(
        out,
        "    <vendorSystickConfig>{}</vendorSystickConfig>",
        chip.vendor_systick_config
    )
    .unwrap();
    writeln!(out, "  </cpu>").unwrap();
    writeln!(out, "  <addressUnitBits>32</addressUnitBits>").unwrap();
    writeln!(out, "  <width>32</width>").unwrap();
    writeln!(out, "  <resetValue>0x00000000</resetValue>").unwrap();
    writeln!(out, "  <resetMask>0xFFFFFFFF</resetMask>").unwrap();
}

/// Loops all peripherals, matches them to instances, writes each block.
fn write_peripherals(out: &mut String, fragment: &RegdescFragment) {
    writeln!(out, "  <peripherals>").unwrap();

    let mut indices: Vec<_> = (0..fragment.peripherals.len()).collect();
    indices.sort_by_key(|&i| fragment.peripherals[i].name.as_str());

    for i in indices {
        let peripheral = &fragment.peripherals[i];
        let mut instances: Vec<_> = fragment
            .peripheral_instances
            .iter()
            .filter(|instance| instance.peripheral == peripheral.name)
            .cloned()
            .collect();

        if instances.is_empty() {
            log::warn!(
                "no instances found for peripheral '{}', skipping",
                peripheral.name
            );
            continue;
        }

        if instances.len() > 1 {
            instances.sort_by(|a, b| a.name.cmp(&b.name));
            write_derived_peripheral(out, fragment, peripheral, &instances);
        } else {
            write_peripheral(out, fragment, peripheral, &instances[0]);
        }
    }

    writeln!(out, "  </peripherals>").unwrap();
}

/// Writes extra instances as `derivedFrom` stubs (same layout, different base
/// addr).
fn write_derived_peripheral(
    out: &mut String,
    fragment: &RegdescFragment,
    peripheral: &Peripheral,
    instances: &[PeripheralInstance],
) {
    let (first, rest) = instances.split_first().unwrap();
    write_peripheral(out, fragment, peripheral, first);

    for instance in rest {
        writeln!(out, "    <peripheral derivedFrom=\"{}\">", first.name).unwrap();
        writeln!(out, "      <name>{}</name>", xml_escape(&instance.name)).unwrap();
        writeln!(
            out,
            "      <baseAddress>{:#x}</baseAddress>",
            instance.base_addr
        )
        .unwrap();
        if let Some(description) = &instance.description {
            writeln!(
                out,
                "      <description>{}</description>",
                xml_escape(description)
            )
            .unwrap();
        }

        // Interrupts are emitted once on the IRQ host peripheral only.
        writeln!(out, "    </peripheral>").unwrap();
    }
}

/// Writes one full peripheral block (name, address, interrupts, registers).
fn write_peripheral(
    out: &mut String,
    fragment: &RegdescFragment,
    peripheral: &Peripheral,
    instance: &PeripheralInstance,
) {
    let description = instance
        .description
        .clone()
        .unwrap_or_else(|| format!("Peripheral {}", instance.name));

    writeln!(out, "    <peripheral>").unwrap();
    writeln!(out, "      <name>{}</name>", xml_escape(&instance.name)).unwrap();
    writeln!(
        out,
        "      <description>{}</description>",
        xml_escape(&description)
    )
    .unwrap();
    writeln!(
        out,
        "      <groupName>{}</groupName>",
        xml_escape(&peripheral.name)
    )
    .unwrap();
    writeln!(
        out,
        "      <baseAddress>{:#x}</baseAddress>",
        instance.base_addr
    )
    .unwrap();

    write_address_block(out, peripheral);

    if is_interrupt_host(fragment, &instance.name) {
        write_instance_interrupts(out, fragment);
    }

    write_registers(out, peripheral);
    writeln!(out, "    </peripheral>").unwrap();
}

/// CMSIS-SVD requires interrupts under some `<peripheral>`; we park the flat
/// list on `INTERRUPT_CORE0` (or the first instance if that is missing).
fn interrupt_host_name(fragment: &RegdescFragment) -> Option<&str> {
    let names: Vec<&str> = fragment
        .peripheral_instances
        .iter()
        .map(|i| i.name.as_str())
        .collect();
    if names.iter().any(|&n| n == "INTERRUPT_CORE0") {
        Some("INTERRUPT_CORE0")
    } else {
        names.first().copied()
    }
}

fn is_interrupt_host(fragment: &RegdescFragment, instance_name: &str) -> bool {
    interrupt_host_name(fragment) == Some(instance_name)
}

fn write_instance_interrupts(out: &mut String, fragment: &RegdescFragment) {
    let mut interrupts = fragment.peripheral_interrupts.clone();
    interrupts.sort_by_key(|i| i.value);
    for interrupt in &interrupts {
        write_interrupt(out, interrupt);
    }
}

/// Writes the `<addressBlock>` size summary.
fn write_address_block(out: &mut String, peripheral: &Peripheral) {
    let size: u32 = peripheral.sorted_registers().iter().map(|r| r.size).sum();
    writeln!(out, "      <addressBlock>").unwrap();
    writeln!(out, "        <offset>0x0</offset>").unwrap();
    writeln!(out, "        <size>{:#x}</size>", size).unwrap();
    writeln!(out, "        <usage>registers</usage>").unwrap();
    writeln!(out, "      </addressBlock>").unwrap();
}

/// Writes one `<interrupt>` entry.
fn write_interrupt(out: &mut String, interrupt: &PeripheralInterrupt) {
    writeln!(out, "      <interrupt>").unwrap();
    writeln!(out, "        <name>{}</name>", xml_escape(&interrupt.name)).unwrap();
    writeln!(out, "        <value>{}</value>", interrupt.value).unwrap();
    if let Some(description) = &interrupt.description {
        writeln!(
            out,
            "        <description>{}</description>",
            xml_escape(description)
        )
        .unwrap();
    }
    writeln!(out, "      </interrupt>").unwrap();
}

/// Writes all registers for a peripheral (name cleanup, dim handling, etc.).
fn write_registers(out: &mut String, peripheral: &Peripheral) {
    writeln!(out, "      <registers>").unwrap();

    let mut registers: Vec<Register> = peripheral
        .register_groups
        .iter()
        .flat_map(|g| g.registers.clone())
        .collect();
    registers.sort_by_key(|r| r.addr);

    for mut register in registers {
        register = register.replace_placeholders("%s");
        register.name = simplify_name(&peripheral.name, &register.name);

        if register.is_mem_region {
            register.name = format!("{}[%s]", register.name);
            register.repeat = Some(Repeat::new(register.size, 1, "n", 0));
            register.size = 1;
        }

        write_register(out, peripheral, &register);
    }

    writeln!(out, "      </registers>").unwrap();
}

/// Writes one `<register>` element.
fn write_register(out: &mut String, peripheral: &Peripheral, register: &Register) {
    writeln!(out, "        <register>").unwrap();
    if let Some(repeat) = &register.repeat {
        if repeat.count > 1 {
            writeln!(out, "          <dim>{}</dim>", repeat.count).unwrap();
            writeln!(
                out,
                "          <dimIncrement>{:#x}</dimIncrement>",
                repeat.stride
            )
            .unwrap();
            // Emit dimIndex whenever the sequence doesn't start at 0 so two
            // arrays that share a `%s` name (e.g. RMT TX `$n` vs RX `$m`)
            // expand to distinct register names (CH0… vs CH2…).
            if repeat.start != 0 {
                let indices: String = (0..repeat.count)
                    .map(|i| (repeat.start + i as i32).to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                writeln!(out, "          <dimIndex>{indices}</dimIndex>").unwrap();
            }
        }
    }
    writeln!(out, "          <name>{}</name>", xml_escape(&register.name)).unwrap();
    writeln!(
        out,
        "          <description>{}</description>",
        xml_escape(register.description.trim())
    )
    .unwrap();
    writeln!(
        out,
        "          <addressOffset>{:#x}</addressOffset>",
        register.addr
    )
    .unwrap();
    writeln!(out, "          <size>{}</size>", register.size * 8).unwrap();

    let reset = register.reset_value();
    if reset != 0 {
        writeln!(out, "          <resetValue>{:#x}</resetValue>", reset).unwrap();
    }

    if !register.fields.is_empty() {
        write_fields(out, peripheral, register);
    }

    writeln!(out, "        </register>").unwrap();
}

/// Writes all `<field>` elements inside a register.
fn write_fields(out: &mut String, peripheral: &Peripheral, register: &Register) {
    writeln!(out, "          <fields>").unwrap();

    for mut field in register.expanded_fields() {
        field.name = simplify_name(&peripheral.name, &field.name);
        if field.name.starts_with(&register.name) && field.name != register.name {
            field.name = simplify_name(&register.name, &field.name);
        }

        if field.name.contains("%s") {
            field.name = field
                .name
                .replace("%s", "")
                .trim_end_matches('_')
                .to_owned();
        }

        let mut description = field.description.clone();
        if FIELD_DESC_INDEX.is_match(&description) {
            description = FIELD_DESC_INDEX
                .replace_all(&description, field.shift.to_string())
                .into_owned();
        }

        writeln!(out, "            <field>").unwrap();
        writeln!(
            out,
            "              <name>{}</name>",
            xml_escape(&field.name)
        )
        .unwrap();
        let desc = if description.trim().is_empty() {
            " ".to_owned()
        } else {
            description
        };
        writeln!(
            out,
            "              <description>{}</description>",
            xml_escape(&desc)
        )
        .unwrap();
        writeln!(out, "              <bitOffset>{}</bitOffset>", field.shift).unwrap();
        writeln!(
            out,
            "              <bitWidth>{}</bitWidth>",
            field.bit_width()
        )
        .unwrap();
        if let Some(access) = guess_field_access(&field.access) {
            writeln!(out, "              <access>{access}</access>").unwrap();
        } else {
            log::warn!(
                "unrecognized field access value '{}', ignoring",
                field.access
            );
        }
        writeln!(out, "            </field>").unwrap();
    }

    writeln!(out, "          </fields>").unwrap();
}

/// Escapes `&`, `<`, `>`, `"` so the XML doesn't break.
fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Gets the current year for the license header (no chrono dependency).
fn time_year() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    // Good enough for license header; avoids pulling in chrono.
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    1970 + (secs / 31_536_000) as i32
}
