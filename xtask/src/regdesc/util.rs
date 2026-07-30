use regex::Regex;
use std::sync::LazyLock;

static VERILOG_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+)'([hdb])([0-9a-fA-F]+)").unwrap());
static ACCESS_SUFFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(/?SC|/?SS|/?WTC|/?WTS)+$").unwrap());

/// Parses Verilog-style literals like `32'hDEAD` into (value, width, base).
pub fn parse_verilog_number(s: &str) -> (u64, u32, u32) {
    let Some(caps) = VERILOG_NUMBER.captures(s) else {
        return (s.parse().unwrap_or(0), 0, 10);
    };

    let width = caps[1].parse().unwrap_or(0);
    let base = match &caps[2] {
        "b" => 2,
        "d" => 10,
        "h" => 16,
        _ => 10,
    };
    let value = u64::from_str_radix(&caps[3], base).unwrap_or(0);
    (value, width, base)
}

/// Finds where two descriptions differ only by an index and replaces it with `$n`.
pub fn remove_index_from_strings(
    a: &str,
    b: &str,
    idx_a: i32,
    idx_b: i32,
    placeholder: &str,
) -> Result<String, String> {
    if a.is_empty() && b.is_empty() {
        return Ok(String::new());
    }
    if a == b {
        return Ok(a.to_owned());
    }

    let idx_a_str = idx_a.to_string();
    let idx_b_str = idx_b.to_string();
    let len_common_prefix = idx_a_str
        .chars()
        .zip(idx_b_str.chars())
        .take_while(|(x, y)| x == y)
        .count();

    let mut pos_a = 0usize;
    let mut pos_b = 0usize;
    let mut res = String::new();
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    while pos_a < a_chars.len() && pos_b < b_chars.len() {
        if a_chars[pos_a] == b_chars[pos_b] {
            res.push(a_chars[pos_a]);
            pos_a += 1;
            pos_b += 1;
            continue;
        }

        let a_suffix: String = a_chars[pos_a.saturating_sub(len_common_prefix)..]
            .iter()
            .collect();
        let b_suffix: String = b_chars[pos_b.saturating_sub(len_common_prefix)..]
            .iter()
            .collect();

        if a_suffix.starts_with(&idx_a_str) && b_suffix.starts_with(&idx_b_str) {
            if len_common_prefix > 0 {
                res.truncate(res.len().saturating_sub(len_common_prefix));
            }
            pos_a += idx_a_str.len() - len_common_prefix;
            pos_b += idx_b_str.len() - len_common_prefix;
            res.push_str(placeholder);
            continue;
        }

        return Err(format!(
            "Strings '{a}' and '{b}' differ at positions {pos_a} and {pos_b}"
        ));
    }

    if pos_a != a_chars.len() || pos_b != b_chars.len() {
        return Err(format!(
            "Strings '{a}' and '{b}' differ at positions {pos_a} and {pos_b}"
        ));
    }

    Ok(res)
}

/// Trims spaces — GDVS CSVs love trailing whitespace.
pub fn trim(s: &str) -> &str {
    s.trim_matches(' ')
}

/// Strips peripheral prefixes so SVD names are shorter (`UART0_TX` → `TX`).
pub fn simplify_name(prefix: &str, name: &str) -> String {
    let prefix_pattern = format!(r"(?i)^{}_", regex::escape(prefix));
    let prefix_re = Regex::new(&prefix_pattern).expect("valid prefix pattern");
    let mut name = prefix_re.replace(name, "").into_owned();

    if prefix.eq_ignore_ascii_case("LP_IO") {
        static LP_GPIO: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^LP_GPIO[\d]*_").unwrap());
        name = LP_GPIO.replace(&name, "").into_owned();
    }

    static REG_SUFFIX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)(_REG)+$").unwrap());
    name = REG_SUFFIX.replace(&name, "").into_owned();

    if name
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit())
    {
        name = format!("_{name}");
    }

    name
}

/// Maps CSV access strings (`RW`, `RO`, …) to CMSIS ones (`read-write`, etc.).
pub fn guess_field_access(access: &str) -> Option<&'static str> {
    const READ_WRITE: &[&str] = &["RW", "RW1C", "RW1S", "W/R", "W1C"];
    const READ_ONLY: &[&str] = &["HRO", "R", "RC", "RO"];
    const WRITE_ONLY: &[&str] = &["W", "WO", "WS", "WT", "WOD", "WOR"];

    /// Does the actual access-string matching (also tries stripping suffixes).
    fn guess(access: &str) -> Option<&'static str> {
        if access.contains("R/W") || READ_WRITE.contains(&access) {
            return Some("read-write");
        }
        if READ_ONLY.contains(&access) {
            return Some("read-only");
        }
        if WRITE_ONLY.contains(&access) {
            return Some("write-only");
        }
        None
    }

    if let Some(access) = guess(access) {
        return Some(access);
    }

    let stripped = ACCESS_SUFFIX.replace(access, "");
    guess(&stripped)
}

/// Turns `esp32s31` into `ESP32-S31` for the SVD header.
pub fn pretty_chip_name(chip: &str) -> String {
    chip.to_ascii_uppercase()
        .replace("ESP32", "ESP32-")
        .trim_matches('-')
        .to_owned()
}
