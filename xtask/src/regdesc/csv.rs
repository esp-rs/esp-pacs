use anyhow::{bail, Context, Result};
use regex::Regex;
use std::sync::LazyLock;

use super::model::{ExpandContext, Field, Peripheral, Register, RegisterGroup};
use super::util::{parse_verilog_number, trim};

static BITPOS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[((\d+):)?(\d+)\]").unwrap());
static NONREG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([0-9]+)\*([0-9]+)").unwrap());
static C_IDENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_$]+$").unwrap());
static REPEAT_LOWER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\$([A-Z])").unwrap());

const CSV_VERSION_1_0: i32 = 100;
const CSV_VERSION_1_1: i32 = 110;
const CSV_VERSION_1_2: i32 = 120;

type CsvLine = std::collections::HashMap<String, String>;

/// Feed it CSV text, get back a parsed `Peripheral`.
pub fn read_peripheral_csv(content: &str, peripheral_name: Option<&str>) -> Result<Peripheral> {
    let mut parser = CsvParser::new(content, peripheral_name)?;
    parser.parse()
}

struct CsvParser {
    lines: Vec<CsvLine>,
    peripheral_name: String,
    csv_version: i32,
    last_repeat_num: Option<i32>,
}

impl CsvParser {
    /// Sets up the parser and figures out the peripheral name if you didn't override it.
    fn new(content: &str, peripheral_name: Option<&str>) -> Result<Self> {
        let (csv_version, records) = parse_csv_records(content)?;
        let peripheral_name = match peripheral_name {
            Some(name) => name.to_owned(),
            None => probe_peripheral_name(&records, "RegName_Chg_0")
                .or_else(|| probe_peripheral_name(&records, "RegName"))
                .context("Failed to determine peripheral name")?,
        };

        Ok(Self {
            lines: records,
            peripheral_name,
            csv_version,
            last_repeat_num: None,
        })
    }

    /// Walks the whole CSV and builds register groups.
    fn parse(&mut self) -> Result<Peripheral> {
        let mut groups: Vec<RegisterGroup> = Vec::new();
        let mut line_idx = 0usize;

        while line_idx < self.lines.len() {
            let (register, group, consumed) = self.parse_register(line_idx)?;
            line_idx += consumed;

            if let Some(idx) = groups.iter().position(|existing| groups_match(existing, &group)) {
                groups[idx].add_register(register);
            } else {
                let mut new_group = group;
                new_group.add_register(register);
                groups.push(new_group);
            }
        }

        for group in &mut groups {
            group.visible = !group.registers.iter().all(|r| !r.visible);
        }

        Ok(Peripheral {
            name: self.peripheral_name.clone(),
            register_groups: groups,
            description: String::new(),
        })
    }

    /// Parses one register line (and the field rows that follow it).
    fn parse_register(&mut self, start: usize) -> Result<(Register, RegisterGroup, usize)> {
        let line = &self.lines[start];
        let reg_name = trim(line.get("RegName").map(String::as_str).unwrap_or(""));
        if reg_name.is_empty() {
            bail!("Expected a register definition at line {}", start + 2);
        }

        let field_name = trim(line.get("Signal").map(String::as_str).unwrap_or(""));
        if !field_name.is_empty() {
            bail!(
                "Field {field_name} cannot be defined on the same line as register {reg_name}"
            );
        }

        let mut reg_name = reg_name.to_owned();
        if let Some(alt) = extract_alt_names(line).into_iter().next() {
            reg_name = alt;
        }

        let description = extract_description(line);
        let visible = extract_visible(line);
        let register_group = extract_register_group(line, self.csv_version);

        let mem_size = extract_mem_size(line, self.csv_version, &reg_name);
        if mem_size == 0 {
            self.validate_register_name(&reg_name)?;

            let repeat_info = self.extract_repeat(line);
            self.last_repeat_num = repeat_info.as_ref().map(|(_, n)| *n);

            let (fields, consumed) = self.parse_fields(start + 1)?;
            if fields.is_empty() {
                bail!("Register {reg_name} doesn't have any fields defined");
            }

            let visible = if fields.iter().all(|f| !f.visible) {
                false
            } else {
                visible
            };

            let mut register = Register {
                name: reg_name,
                addr: extract_reg_addr(line)?,
                fields,
                description,
                visible,
                size: 4,
                repeat: None,
                repeat_name_hint: None,
                repeat_index_hint: None,
                is_mem_region: false,
                expand_context: ExpandContext::default(),
            };

            if let Some((repeat_name, repeat_num)) = repeat_info {
                register.set_repeat_hint(repeat_name, repeat_num);
            }

            Ok((register, register_group, consumed + 1))
        } else {
            self.validate_mem_region_name(&reg_name)?;
            let register = Register {
                name: reg_name,
                addr: extract_reg_addr(line)?,
                fields: Vec::new(),
                description,
                visible: true,
                size: mem_size,
                repeat: None,
                repeat_name_hint: None,
                repeat_index_hint: None,
                is_mem_region: true,
                expand_context: ExpandContext::default(),
            };
            Ok((register, register_group, 1))
        }
    }

    /// Grabs all field rows belonging to the current register.
    fn parse_fields(&self, start: usize) -> Result<(Vec<Field>, usize)> {
        let mut fields = Vec::new();
        let mut idx = start;

        while idx < self.lines.len() {
            let line = &self.lines[idx];
            if !trim(line.get("RegName").map(String::as_str).unwrap_or("")).is_empty() {
                break;
            }

            let field_name = trim(line.get("Signal").map(String::as_str).unwrap_or(""));
            if field_name.is_empty() {
                if extract_field_shift_mask(line).is_some() {
                    idx += 1;
                    continue;
                }
                break;
            }

            let mut field_name = field_name.to_ascii_uppercase();
            if let Some(alt) = extract_alt_names(line).into_iter().next() {
                field_name = alt;
            }
            field_name = self.validate_fix_field_name(&field_name)?;

            let (shift, mask) = extract_field_shift_mask(line)
                .context("Field missing bit position")?;
            let (min_val, max_val) = extract_min_max(line);
            let (default, _, _) = parse_verilog_number(
                trim(line.get("Default").map(String::as_str).unwrap_or("0")),
            );

            let mut field = Field {
                name: field_name,
                shift,
                mask,
                access: trim(line.get("SW(R/W)").map(String::as_str).unwrap_or(""))
                    .to_owned(),
                default,
                description: extract_description(line),
                visible: extract_visible(line),
                min_val,
                max_val,
                repeat: None,
                repeat_name_hint: None,
                repeat_index_hint: None,
            };

            if let Some((repeat_name, repeat_num)) = self.extract_repeat(line) {
                field.set_repeat_hint(repeat_name, repeat_num);
            }

            fields.push(field);
            idx += 1;
        }

        Ok((fields, idx - start))
    }

    /// Pulls out repeat info like `$n` and how many times it repeats.
    fn extract_repeat(&self, line: &CsvLine) -> Option<(String, i32)> {
        let repeat_name = trim(line.get("RepeatName").map(String::as_str).unwrap_or(""));
        if repeat_name.is_empty() {
            return None;
        }

        let mut repeat_name = repeat_name.to_ascii_uppercase();
        repeat_name = REPEAT_LOWER
            .replace_all(&repeat_name, |caps: &regex::Captures| {
                format!("${}", caps[1].to_ascii_lowercase())
            })
            .into_owned();

        let num_str = trim(line.get("RepeatNum").map(String::as_str).unwrap_or(""));
        if num_str.is_empty() {
            return self.last_repeat_num.map(|n| (repeat_name, n));
        }

        Some((repeat_name, num_str.parse().ok()?))
    }

    /// Warns if a register name looks wrong (doesn't stop parsing).
    fn validate_register_name(&self, reg_name: &str) -> Result<()> {
        if !C_IDENT_REGEX.is_match(reg_name) {
            log::warn!("Invalid register name: {reg_name}");
        }
        let prefix = format!("{}_", self.peripheral_name);
        if !reg_name.starts_with(&prefix) || !reg_name.ends_with("_REG") {
            log::warn!("Invalid register name: {reg_name}");
        }
        Ok(())
    }

    /// Same as `validate_register_name`, but for `*_MEM` memory regions.
    fn validate_mem_region_name(&self, name: &str) -> Result<()> {
        if !C_IDENT_REGEX.is_match(name) {
            log::warn!("Invalid memory region name: {name}");
        }
        let prefix = format!("{}_", self.peripheral_name);
        if !name.starts_with(&prefix) || !name.ends_with("_MEM") {
            log::warn!("Invalid memory region name: {name}");
        }
        Ok(())
    }

    /// Tries to fix/normalize field names; warns if the prefix looks off.
    fn validate_fix_field_name(&self, field_name: &str) -> Result<String> {
        if !C_IDENT_REGEX.is_match(field_name) {
            log::warn!("Invalid field name: {field_name}");
        }

        let prefix_variants = if self.csv_version < CSV_VERSION_1_1 {
            vec![
                format!("REG_{}_REG_", self.peripheral_name),
                format!("REG_{}_", self.peripheral_name),
            ]
        } else {
            vec![format!("{}_", self.peripheral_name)]
        };

        for prefix in &prefix_variants {
            if let Some(rest) = field_name.strip_prefix(prefix) {
                return Ok(format!("{}_{rest}", self.peripheral_name));
            }
        }

        let prefix_list = prefix_variants.join(" or ");
        log::warn!(
            "Invalid field name: {field_name}, expected prefix {prefix_list}; keeping original"
        );
        Ok(field_name.to_owned())
    }
}

/// Checks if two register groups should be merged into one.
fn groups_match(existing: &RegisterGroup, group: &RegisterGroup) -> bool {
    if existing.name.is_none() && group.name.is_none() {
        return existing.description == group.description;
    }
    existing.name == group.name && existing.description == group.description
}

/// Low-level CSV reader (semicolon-delimited, GDVS style).
fn parse_csv_records(content: &str) -> Result<(i32, Vec<CsvLine>)> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .context("CSV missing headers")?
        .iter()
        .map(|h| trim(h).to_owned())
        .collect();

    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.context("invalid CSV record")?;
        let mut line = CsvLine::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            line.insert(header.clone(), value.to_owned());
        }
        records.push(line);
    }

    let csv_version = if headers.iter().any(|h| h == "CSV_Version") {
        match records.first().and_then(|line| line.get("CSV_Version")) {
            Some(v) => match trim(v) {
                "1.0" => CSV_VERSION_1_0,
                "1.1" => CSV_VERSION_1_1,
                "1.2" => CSV_VERSION_1_2,
                other => bail!("Unknown CSV version: {other}"),
            },
            None => CSV_VERSION_1_0,
        }
    } else {
        CSV_VERSION_1_0
    };

    Ok((csv_version, records))
}

/// Guesses the peripheral name from common prefixes in register names.
fn probe_peripheral_name(records: &[CsvLine], column: &str) -> Option<String> {
    let names: Vec<String> = records
        .iter()
        .filter(|line| !trim(line.get("Address").map(String::as_str).unwrap_or("")).is_empty())
        .filter_map(|line| line.get(column).cloned())
        .filter(|name| !name.is_empty())
        .collect();

    if names.len() < 2 {
        return None;
    }

    let common_prefix = common_prefix(&names);
    let pos = common_prefix.rfind('_')?;
    Some(common_prefix[..pos].to_owned())
}

/// Finds the shared start of a bunch of strings (used for name probing).
fn common_prefix(values: &[String]) -> String {
    if values.is_empty() {
        return String::new();
    }
    let mut prefix = values[0].clone();
    for value in &values[1..] {
        while !value.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

/// Parses the address column from a CSV row.
fn extract_reg_addr(line: &CsvLine) -> Result<u32> {
    let addr = trim(line.get("Address").map(String::as_str).unwrap_or(""));
    let hex = addr
        .strip_prefix("0x")
        .or_else(|| addr.strip_prefix("0X"))
        .unwrap_or(addr);
    u32::from_str_radix(hex, 16).with_context(|| format!("Invalid address: {addr}"))
}

/// Figures out memory region size from the `NonReg` column.
fn extract_mem_size(line: &CsvLine, csv_version: i32, reg_name: &str) -> u32 {
    let nonreg_val = trim(line.get("NonReg").map(String::as_str).unwrap_or(""));
    if nonreg_val.is_empty() {
        return 0;
    }

    if csv_version < CSV_VERSION_1_2 {
        return u32::from_str_radix(nonreg_val.trim_start_matches("0x"), 16).unwrap_or(0);
    }

    if let Some(caps) = NONREG_REGEX.captures(nonreg_val) {
        let depth = caps[1].parse::<u32>().unwrap_or(0);
        let width_bits = caps[2].parse::<u32>().unwrap_or(0);
        if width_bits != 32 {
            log::warn!(
                "Memory region {reg_name} bit width {width_bits} isn't implemented, assuming 32"
            );
        }
        return depth * 4;
    }

    0
}

/// Grabs the description text from a row.
fn extract_description(line: &CsvLine) -> String {
    trim(line.get("Description").map(String::as_str).unwrap_or("")).to_owned()
}

/// Checks if a row is marked public/visible (empty `Public` column = yes).
fn extract_visible(line: &CsvLine) -> bool {
    matches!(
        trim(line.get("Public").map(String::as_str).unwrap_or("")),
        ""
    )
}

/// Pulls alternate register/field names from `RegName_Chg_0`.
fn extract_alt_names(line: &CsvLine) -> Vec<String> {
    let alt = trim(line.get("RegName_Chg_0").map(String::as_str).unwrap_or(""));
    if alt.is_empty() {
        Vec::new()
    } else {
        alt.split_whitespace().map(|s| s.to_ascii_uppercase()).collect()
    }
}

/// Reads min/max value columns if present.
fn extract_min_max(line: &CsvLine) -> (Option<i64>, Option<i64>) {
    let min_str = trim(line.get("Min_Value").map(String::as_str).unwrap_or(""));
    let max_str = trim(line.get("Max_Value").map(String::as_str).unwrap_or(""));
    let min_val = if min_str.is_empty() {
        None
    } else {
        min_str.parse().ok()
    };
    let max_val = if max_str.is_empty() {
        None
    } else {
        max_str.parse().ok()
    };
    (min_val, max_val)
}

/// Builds a register group from `GroupName` / `GroupIdentifier`.
fn extract_register_group(line: &CsvLine, csv_version: i32) -> RegisterGroup {
    let group_desc = trim(line.get("GroupName").map(String::as_str).unwrap_or(""));
    let description = if group_desc.is_empty() {
        "Default".to_owned()
    } else {
        group_desc.to_owned()
    };
    let name = if csv_version < CSV_VERSION_1_2 {
        None
    } else {
        let id = trim(line.get("GroupIdentifier").map(String::as_str).unwrap_or(""));
        if id.is_empty() {
            None
        } else {
            Some(id.to_owned())
        }
    };

    RegisterGroup {
        name,
        description,
        registers: Vec::new(),
        visible: true,
        repeat: None,
        offset: 0,
    }
}

/// Parses `[7:0]`-style bit positions into (shift, mask).
fn extract_field_shift_mask(line: &CsvLine) -> Option<(u32, u32)> {
    let bitpos = trim(line.get("BitPos").map(String::as_str).unwrap_or(""));
    if bitpos.is_empty() {
        return None;
    }
    let caps = BITPOS_REGEX.captures(bitpos)?;
    if caps.get(2).is_none() {
        let bit = caps[3].parse().ok()?;
        return Some((bit, 0x1));
    }
    let high = caps[2].parse::<u32>().ok()?;
    let low = caps[3].parse::<u32>().ok()?;
    let width = high - low + 1;
    let mask = if width >= 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    Some((low, mask))
}
