use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

use super::{
    model::{ExpandContext, ExpandValue, Field, Register, Repeat},
    util::remove_index_from_strings,
};

static INDEX_VAR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$([a-zA-Z])").unwrap());

#[derive(Debug, Clone)]
pub struct MergeError(pub String);

/// Collapses repeated field rows (CH0, CH1, …) into one field with a dim array.
pub fn merge_fields(fields: Vec<Field>) -> (Vec<Field>, Vec<MergeError>) {
    let mut items_by_repeat_name: HashMap<String, Vec<Field>> = HashMap::new();
    let mut result = Vec::new();
    let mut merge_errors = Vec::new();

    for mut field in fields {
        match (&field.repeat_name_hint, field.repeat_index_hint) {
            (Some(name), Some(_)) => {
                items_by_repeat_name
                    .entry(name.clone())
                    .or_default()
                    .push(field);
            }
            (Some(name), None) => {
                field.name = name.clone();
                field.repeat_name_hint = None;
                result.push(field);
            }
            _ => result.push(field),
        }
    }

    for (repeat_name, mut items_list) in items_by_repeat_name {
        items_list.sort_by_key(|f| f.repeat_index_hint.unwrap_or(0));
        match merge_field_group(&mut items_list, &repeat_name) {
            Ok(merged) => result.push(merged),
            Err(err) => {
                merge_errors.push(err);
                result.extend(items_list);
            }
        }
    }

    result.sort_by_key(|f| f.shift);
    (result, merge_errors)
}

/// Same as `merge_fields`, but for registers.
pub fn merge_registers(registers: Vec<Register>) -> (Vec<Register>, Vec<MergeError>) {
    let mut items_by_repeat_name: HashMap<String, Vec<Register>> = HashMap::new();
    let mut result = Vec::new();
    let mut merge_errors = Vec::new();

    for mut register in registers {
        match (&register.repeat_name_hint, register.repeat_index_hint) {
            (Some(name), Some(_)) => {
                items_by_repeat_name
                    .entry(name.clone())
                    .or_default()
                    .push(register);
            }
            (Some(name), None) => {
                register.name = name.clone();
                register.repeat_name_hint = None;
                result.push(register);
            }
            _ => result.push(register),
        }
    }

    for (repeat_name, mut items_list) in items_by_repeat_name {
        items_list.sort_by_key(|r| r.repeat_index_hint.unwrap_or(0));
        match merge_register_group(&mut items_list, &repeat_name) {
            Ok(merged) => result.push(merged),
            Err(err) => {
                merge_errors.push(err);
                result.extend(items_list);
            }
        }
    }

    result.sort_by_key(|r| r.addr);
    (result, merge_errors)
}

/// Actually merges one group of repeated fields into a single dim'd field.
fn merge_field_group(fields: &mut [Field], repeat_name: &str) -> Result<Field, MergeError> {
    let start = merge_get_start(fields, repeat_name)?;
    for field in &fields[1..] {
        if field.mask != fields[0].mask
            || field.access != fields[0].access
            || field.default != fields[0].default
            || field.visible != fields[0].visible
            || field.min_val != fields[0].min_val
            || field.max_val != fields[0].max_val
        {
            return Err(MergeError(format!(
                "Field {} differs from {}",
                field.name, fields[0].name
            )));
        }
    }
    let stride = merge_get_stride(repeat_name, fields, |f| f.shift)?;
    let index_var = merge_get_index_var(repeat_name)?;
    merge_validate_names(fields, &index_var, repeat_name, |f| &f.name)?;
    merge_validate_descriptions(
        fields,
        &index_var,
        |f| &f.name,
        |f| f.description.clone(),
        |f, d| f.description = d,
    )?;

    let mut merged = fields[0].clone();
    merged.name = repeat_name.to_owned();
    merged.repeat = Some(Repeat::new(fields.len() as u32, stride, index_var, start));
    merged.repeat_name_hint = None;
    merged.repeat_index_hint = None;
    Ok(merged)
}

/// Actually merges one group of repeated registers into a single dim'd
/// register.
fn merge_register_group(
    registers: &mut [Register],
    repeat_name: &str,
) -> Result<Register, MergeError> {
    let start = merge_get_start(registers, repeat_name)?;
    for register in &registers[1..] {
        if register.visible != registers[0].visible {
            return Err(MergeError(format!(
                "Register {} visible differs from {}",
                register.name, registers[0].name
            )));
        }
    }
    let stride = merge_get_stride(repeat_name, registers, |r| r.addr)?;
    let index_var = merge_get_index_var(repeat_name)?;
    merge_validate_names(registers, &index_var, repeat_name, |r| &r.name)?;
    merge_validate_descriptions(
        registers,
        &index_var,
        |r| &r.name,
        |r| r.description.clone(),
        |r, d| r.description = d,
    )?;

    let mut merged = registers[0].clone();
    merged.name = repeat_name.to_owned();
    merged.repeat = Some(Repeat::new(
        registers.len() as u32,
        stride,
        index_var,
        start,
    ));
    merged.repeat_name_hint = None;
    merged.repeat_index_hint = None;
    Ok(merged)
}

/// Checks repeat indices are consecutive (0, 1, 2 — not 0, 2, 5).
fn merge_get_start<T>(items_list: &[T], repeat_name: &str) -> Result<i32, MergeError>
where
    T: RepeatHint,
{
    let value_0 = items_list[0].repeat_index_hint().unwrap_or(0);
    for (idx, item) in items_list.iter().enumerate() {
        if item.repeat_index_hint() != Some(value_0 + idx as i32) {
            let indices: Vec<_> = items_list
                .iter()
                .map(|i| i.repeat_index_hint().unwrap_or(-1))
                .collect();
            return Err(MergeError(format!(
                "Repeat indices for {repeat_name} should be consecutive integers starting from {value_0}. Got {indices:?}"
            )));
        }
    }
    Ok(value_0)
}

/// Computes the address/bit spacing between repeated items.
fn merge_get_stride<T, F>(
    repeat_name: &str,
    items_list: &[T],
    get_offset: F,
) -> Result<u32, MergeError>
where
    F: Fn(&T) -> u32,
{
    if items_list.len() <= 1 {
        return Ok(0);
    }
    let offsets: Vec<u32> = items_list.iter().map(get_offset).collect();
    let offset_0 = offsets[0];
    let mut offsets_from_0 = Vec::with_capacity(offsets.len());
    for &offset in &offsets {
        let Some(delta) = offset.checked_sub(offset_0) else {
            return Err(MergeError(format!(
                "Repeat offsets for {repeat_name} are not monotonically increasing. Got {offsets:?}"
            )));
        };
        offsets_from_0.push(delta);
    }
    let stride = offsets_from_0[1];
    if !offsets_from_0
        .iter()
        .enumerate()
        .all(|(idx, val)| *val == idx as u32 * stride)
    {
        return Err(MergeError(format!(
            "Repeat offsets for {repeat_name} should be equally spaced. Got {offset_0} + {offsets_from_0:?}"
        )));
    }
    Ok(stride)
}

/// Pulls the `$n` (or `$a`) variable out of a repeat name.
fn merge_get_index_var(repeat_name: &str) -> Result<String, MergeError> {
    let index_vars: Vec<_> = INDEX_VAR_REGEX
        .captures_iter(repeat_name)
        .map(|c| c[1].to_owned())
        .collect();
    if index_vars.is_empty() {
        return Err(MergeError(format!(
            "No index vars found in name: {repeat_name}"
        )));
    }
    if index_vars.len() > 1 {
        return Err(MergeError(format!(
            "Multiple index vars not supported yet ({repeat_name})"
        )));
    }
    Ok(index_vars[0].clone())
}

/// Makes sure each item's real name matches what the repeat pattern expects.
fn merge_validate_names<T, F>(
    items_list: &[T],
    index_var: &str,
    repeat_name: &str,
    get_name: F,
) -> Result<(), MergeError>
where
    T: RepeatHint,
    F: Fn(&T) -> &str,
{
    let start = items_list[0].repeat_index_hint().unwrap_or(0);
    for (idx, item) in items_list.iter().enumerate() {
        let mut ctx = ExpandContext::default();
        ctx.insert(index_var.to_owned(), ExpandValue::Int(start + idx as i32));
        let expanded_name = ctx.replace(repeat_name);
        let real_name = get_name(item);
        if !expanded_name.contains(real_name) {
            return Err(MergeError(format!(
                "Name {real_name} doesn't match repeat name {repeat_name} with index {}",
                start + idx as i32
            )));
        }
    }
    Ok(())
}

/// Tries to merge descriptions that only differ by the index number.
fn merge_validate_descriptions<T>(
    items_list: &mut [T],
    index_var: &str,
    get_name: impl Fn(&T) -> &str,
    get_desc: impl Fn(&T) -> String,
    set_desc: impl Fn(&mut T, String),
) -> Result<(), MergeError>
where
    T: RepeatHint + Clone,
{
    if items_list.len() < 2 {
        return Ok(());
    }

    let item_0 = items_list[0].clone();
    let item_0_index = items_list[0].repeat_index_hint().unwrap_or(0);
    let item_0_value = get_desc(&item_0);
    let mut new_value = String::new();
    let mut replaced = vec![false; items_list.len()];

    for (i, item) in items_list.iter().enumerate() {
        let item_value = get_desc(item);
        let item_index = item.repeat_index_hint().unwrap_or(i as i32);
        if item_value == item_0_value {
            continue;
        }
        let candidate = remove_index_from_strings(
            &item_0_value,
            &item_value,
            item_0_index,
            item_index,
            &format!("${index_var}"),
        )
        .map_err(MergeError)?;
        if !new_value.is_empty() && new_value != candidate {
            return Err(MergeError(format!(
                "Failed to merge description of {} and {}",
                get_name(&item_0),
                get_name(item)
            )));
        }
        new_value = candidate;
        replaced[i] = true;
    }

    if replaced.iter().skip(1).all(|&x| !x) {
        return Ok(());
    }
    if replaced.iter().skip(1).all(|&x| x) {
        for item in items_list.iter_mut() {
            set_desc(item, new_value.clone());
        }
        return Ok(());
    }

    Err(MergeError(format!(
        "Failed to merge description of {}",
        get_name(&item_0)
    )))
}

/// Lets merge helpers read the repeat index from a field or register.
trait RepeatHint {
    fn repeat_index_hint(&self) -> Option<i32>;
}

impl RepeatHint for Field {
    fn repeat_index_hint(&self) -> Option<i32> {
        self.repeat_index_hint
    }
}

impl RepeatHint for Register {
    fn repeat_index_hint(&self) -> Option<i32> {
        self.repeat_index_hint
    }
}
