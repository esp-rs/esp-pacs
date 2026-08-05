use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;

use super::model::{
    ChipInfo,
    Peripheral,
    PeripheralInstance,
    PeripheralInterrupt,
    RegdescFragment,
};

#[derive(Debug, Deserialize)]
struct DescriptionsFile {
    peripheral_descriptions: Vec<PeripheralDescription>,
}

#[derive(Debug, Deserialize)]
struct PeripheralDescription {
    peripheral: String,
    description: String,
}

/// Lists `*.csv` files in `csv_dir` (sorted by file name).
pub fn discover_csv_files(csv_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(csv_dir)
        .with_context(|| format!("reading CSV directory {}", csv_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("csv") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// Loads the shared peripheral description blurbs from YAML.
pub fn load_descriptions(path: &Path) -> Result<HashMap<String, String>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let file: DescriptionsFile =
        serde_yaml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
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
        instance.description = descriptions.get(&instance.name).cloned().or_else(|| {
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

/// Path to `xtask/regdesc/`.
pub fn regdesc_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("regdesc")
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
