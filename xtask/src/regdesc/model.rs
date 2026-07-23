use std::collections::HashMap;

use super::merge::{merge_fields, merge_registers, MergeError};

#[derive(Debug, Clone)]
pub struct Repeat {
    pub count: u32,
    pub stride: u32,
    pub index_var: String,
    pub start: i32,
}

impl Repeat {
    /// Creates a repeat spec (count, stride, index var, start index).
    pub fn new(count: u32, stride: u32, index_var: impl Into<String>, start: i32) -> Self {
        Self {
            count,
            stride,
            index_var: index_var.into(),
            start,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpandContext {
    values: HashMap<String, ExpandValue>,
}

#[derive(Debug, Clone)]
pub enum ExpandValue {
    Int(i32),
    Str(String),
}

impl ExpandContext {
    /// Stores a value for a `$placeholder`.
    pub fn insert(&mut self, key: impl Into<String>, value: ExpandValue) {
        self.values.insert(key.into(), value);
    }

    /// Substitutes `$n` etc. in a string with actual values.
    pub fn replace(&self, text: &str) -> String {
        let mut text = text.to_owned();
        for (placeholder, value) in &self.values {
            let token = format!("${placeholder}");
            match value {
                ExpandValue::Str(subst) => {
                    text = text.replace(&token, subst);
                }
                ExpandValue::Int(n) => {
                    text = text.replace(&token, &n.to_string());
                }
            }
        }
        text
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub shift: u32,
    pub mask: u32,
    pub access: String,
    pub default: u64,
    pub description: String,
    pub visible: bool,
    pub min_val: Option<i64>,
    pub max_val: Option<i64>,
    pub repeat: Option<Repeat>,
    pub repeat_name_hint: Option<String>,
    pub repeat_index_hint: Option<i32>,
}

impl Field {
    /// How many bits this field covers (counts ones in the mask).
    pub fn bit_width(&self) -> u32 {
        self.mask.count_ones()
    }

    /// Marks this field as "item N of a repeat group".
    pub fn set_repeat_hint(&mut self, name: String, index: i32) {
        self.repeat_name_hint = Some(name);
        self.repeat_index_hint = Some(index);
    }

    /// Spreads a repeated field into individual copies for SVD output.
    pub fn expand(&self, ctx: &ExpandContext) -> Vec<Field> {
        let repeat = self
            .repeat
            .clone()
            .unwrap_or_else(|| Repeat::new(1, 0, "n", 0));
        let mut result = Vec::new();
        for i in 0..repeat.count {
            let mut field = self.clone();
            field.repeat = None;
            field.shift += repeat.stride * i;
            let mut expand_ctx = ctx.clone();
            if self.repeat.is_some() && !expand_ctx.values.contains_key(&repeat.index_var) {
                expand_ctx.insert(repeat.index_var.clone(), ExpandValue::Int(i as i32 + repeat.start));
            }
            field.name = expand_ctx.replace(&field.name);
            field.description = expand_ctx.replace(&field.description);
            result.push(field);
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub addr: u32,
    pub fields: Vec<Field>,
    pub description: String,
    pub visible: bool,
    pub size: u32,
    pub repeat: Option<Repeat>,
    pub repeat_name_hint: Option<String>,
    pub repeat_index_hint: Option<i32>,
    pub is_mem_region: bool,
    pub expand_context: ExpandContext,
}

impl Register {
    /// Computes the register reset value from all field defaults OR'd together.
    pub fn reset_value(&self) -> u64 {
        self.fields
            .iter()
            .fold(0u64, |acc, field| acc | (field.default << field.shift))
    }

    /// Marks this register as "item N of a repeat group".
    pub fn set_repeat_hint(&mut self, name: String, index: i32) {
        self.repeat_name_hint = Some(name);
        self.repeat_index_hint = Some(index);
    }

    /// Runs field merging on this register.
    pub fn merge_fields(&mut self) -> Vec<MergeError> {
        let (merged, errors) = merge_fields(std::mem::take(&mut self.fields));
        self.fields = merged;
        errors
    }

    /// Returns all fields after repeat expansion.
    pub fn expanded_fields(&self) -> Vec<Field> {
        let ctx = &self.expand_context;
        let mut fields = Vec::new();
        for field in &self.fields {
            fields.extend(field.expand(ctx));
        }
        fields.sort_by_key(|f| f.shift);
        fields
    }

    /// Spreads a repeated register into individual copies.
    pub fn expand(&self, ctx: &ExpandContext) -> Vec<Register> {
        let repeat = self
            .repeat
            .clone()
            .unwrap_or_else(|| Repeat::new(1, 0, "n", 0));
        let mut result = Vec::new();
        for i in 0..repeat.count {
            let mut reg = self.clone();
            reg.repeat = None;
            reg.addr += repeat.stride * i;
            let mut expand_ctx = ctx.clone();
            if self.repeat.is_some() && !expand_ctx.values.contains_key(&repeat.index_var) {
                expand_ctx.insert(repeat.index_var.clone(), ExpandValue::Int(i as i32 + repeat.start));
            }
            reg.name = expand_ctx.replace(&reg.name);
            reg.description = expand_ctx.replace(&reg.description);
            result.push(reg);
        }
        result
    }

    /// Swaps `$n` for `%s` (SVD dim syntax) before writing.
    pub fn replace_placeholders(&self, sub_expr: &str) -> Register {
        let mut reg = self.clone();
        if let Some(repeat) = &self.repeat {
            let mut ctx = ExpandContext::default();
            ctx.insert(repeat.index_var.clone(), ExpandValue::Str(sub_expr.to_owned()));
            reg.expand_context = ctx.clone();
            reg.name = ctx.replace(&reg.name);
            reg.description = ctx.replace(&reg.description);
        }
        reg
    }
}

#[derive(Debug, Clone)]
pub struct RegisterGroup {
    pub name: Option<String>,
    pub description: String,
    pub registers: Vec<Register>,
    pub visible: bool,
    pub repeat: Option<Repeat>,
    pub offset: u32,
}

impl RegisterGroup {
    /// Adds a register and keeps them sorted by address.
    pub fn add_register(&mut self, reg: Register) {
        self.registers.push(reg);
        self.registers.sort_by_key(|r| r.addr);
    }

    /// Runs register merging within this group.
    pub fn merge_registers(&mut self) -> Vec<MergeError> {
        let (merged, errors) = merge_registers(std::mem::take(&mut self.registers));
        self.registers = merged;
        errors
    }

    /// Returns all registers after group + register repeat expansion.
    pub fn expanded_registers(&self) -> Vec<Register> {
        let groups = if self.repeat.is_some() {
            self.expand(&ExpandContext::default())
        } else {
            vec![self.clone()]
        };

        let mut result = Vec::new();
        for group in groups {
            for register in &group.registers {
                let expanded = register.expand(&ExpandContext::default());
                for mut reg in expanded {
                    reg.addr += group.offset;
                    result.push(reg);
                }
            }
        }
        result.sort_by_key(|r| r.addr);
        result
    }

    /// Expands a repeated register group into copies.
    fn expand(&self, ctx: &ExpandContext) -> Vec<RegisterGroup> {
        let repeat = self
            .repeat
            .clone()
            .unwrap_or_else(|| Repeat::new(1, 0, "n", 0));
        let mut result = Vec::new();
        for i in 0..repeat.count {
            let mut group = self.clone();
            group.repeat = None;
            group.offset += repeat.stride * i;
            let mut expand_ctx = ctx.clone();
            if self.repeat.is_some() && !expand_ctx.values.contains_key(&repeat.index_var) {
                expand_ctx.insert(repeat.index_var.clone(), ExpandValue::Int(i as i32 + repeat.start));
            }
            if let Some(name) = &group.name {
                group.name = Some(expand_ctx.replace(name));
            }
            result.push(group);
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub register_groups: Vec<RegisterGroup>,
    #[allow(dead_code)]
    pub description: String,
}

impl Peripheral {
    /// Runs merge on all registers and fields in this peripheral.
    pub fn merge_registers_fields(&mut self) -> Vec<MergeError> {
        let mut errors = Vec::new();
        for group in &mut self.register_groups {
            errors.extend(group.merge_registers());
            for register in &mut group.registers {
                errors.extend(register.merge_fields());
            }
        }
        errors
    }

    /// Flat list of all registers, sorted by address.
    pub fn sorted_registers(&self) -> Vec<Register> {
        let mut all: Vec<_> = self
            .register_groups
            .iter()
            .flat_map(|g| g.expanded_registers())
            .collect();
        all.sort_by_key(|r| r.addr);
        all
    }
}

#[derive(Debug, Clone)]
pub struct PeripheralInstance {
    pub name: String,
    pub peripheral: String,
    pub base_addr: u32,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PeripheralInterrupt {
    pub name: String,
    pub instance: String,
    pub value: u32,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChipInfo {
    pub name: String,
    pub description: String,
    pub cpu_name: String,
    pub cpu_revision: String,
    pub cpu_endian: String,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_prio_bits: u32,
    pub vendor_systick_config: bool,
}

impl ChipInfo {
    /// Builds chip metadata (CPU type, description, FPU, etc.) from `"esp32s31"`.
    pub fn from_chip_id(chip: &str) -> Self {
        let formal = super::util::pretty_chip_name(chip);
        let description = match formal.as_str() {
            "ESP32" => "32-bit MCU & 2.4 GHz Wi-Fi & Bluetooth/Bluetooth LE",
            "ESP32-C2" => "32-bit RISC-V MCU & 2.4 GHz Wi-Fi & Bluetooth 5 (LE)",
            "ESP32-C3" => "32-bit RISC-V MCU & 2.4 GHz Wi-Fi & Bluetooth 5 (LE)",
            "ESP32-C5" => {
                "32-bit RISC-V MCU & 2.4 and 5 GHz Wi-Fi 6 & Bluetooth 5 (LE) & IEEE 802.15.4"
            }
            "ESP32-C6" => {
                "32-bit RISC-V MCU & 2.4 GHz Wi-Fi 6 & Bluetooth 5 (LE) & IEEE 802.15.4"
            }
            "ESP32-C6-LP" => "32-bit RISC-V MCU",
            "ESP32-C61" => "32-bit RISC-V MCU & 2.4 GHz Wi-Fi 6 & Bluetooth 5 (LE)",
            "ESP32-H2" => "32-bit RISC-V MCU & Bluetooth 5 (LE) & IEEE 802.15.4",
            "ESP32-P4" => "32-bit RISC-V MCU",
            "ESP32-S2" => "32-bit MCU & 2.4 GHz Wi-Fi",
            "ESP32-S2-ULP" => "32-bit RISC-V MCU",
            "ESP32-S3" => "32-bit MCU & 2.4 GHz Wi-Fi & Bluetooth 5 (LE)",
            "ESP32-S3-ULP" => "32-bit RISC-V MCU",
            "ESP32-S31" => {
                "32-bit RISC-V MCU & 2.4 GHz Wi-Fi & Bluetooth 5 (LE) & IEEE 802.15.4"
            }
            _ => "No Chip Description",
        }
        .to_owned();

        let cpu_name = match formal.as_str() {
            "ESP32" => "Xtensa LX6",
            "ESP32-S2" | "ESP32-S3" => "Xtensa LX7",
            "ESP32-C2" | "ESP32-C3" | "ESP32-S2-ULP" | "ESP32-S3-ULP" => "RV32IMC",
            "ESP32-P4" | "ESP32-S31" => "RV32IMAFC",
            _ => "RV32IMAC",
        }
        .to_owned();

        let fpu_present = matches!(
            formal.as_str(),
            "ESP32" | "ESP32-P4" | "ESP32-S3" | "ESP32-S31"
        );

        Self {
            name: formal,
            description,
            cpu_name,
            cpu_revision: "r0p0".to_owned(),
            cpu_endian: "little".to_owned(),
            mpu_present: false,
            fpu_present,
            nvic_prio_bits: 0,
            vendor_systick_config: false,
        }
    }

    /// Returns the marketing series string like `"ESP32 S-Series"`.
    pub fn series(&self) -> &str {
        if self.name.contains("ULP") {
            "RISC-V ULP"
        } else if self.name.starts_with("ESP32-C") {
            "ESP32 C-Series"
        } else if self.name.starts_with("ESP32-H") {
            "ESP32 H-Series"
        } else if self.name.starts_with("ESP32-P") {
            "ESP32 P-Series"
        } else if self.name.starts_with("ESP32-S") {
            "ESP32 S-Series"
        } else {
            "ESP32 Series"
        }
    }
}

#[derive(Debug)]
pub struct RegdescFragment {
    pub peripherals: Vec<Peripheral>,
    pub peripheral_instances: Vec<PeripheralInstance>,
    pub peripheral_interrupts: Vec<PeripheralInterrupt>,
    pub chip_info: ChipInfo,
}
