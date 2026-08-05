mod config;
mod csv;
mod idf;
mod merge;
mod model;
mod svd;
mod util;

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
pub use config::default_output_path;
use config::{discover_csv_files, load_descriptions, peripheral_descriptions_path};
use model::RegdescFragment;

/// IDF headers that supply peripheral bases and the IRQ list.
#[derive(Debug, Clone)]
pub struct GenerateSources {
    /// Path to `soc/interrupts.h`.
    pub interrupts: PathBuf,
    /// Path to `register/soc/reg_base.h` (or chip-equivalent).
    pub reg_base: PathBuf,
}

/// Main entry: load IDF metadata + CSVs, build the chip model, write an SVD
/// file.
pub fn generate_base_svd(
    chip: &str,
    csv_dir: &Path,
    output: &Path,
    version: u32,
    sources: &GenerateSources,
) -> Result<()> {
    log::info!(
        "Generating base SVD for {chip} (version {version}) using {} and {}",
        sources.reg_base.display(),
        sources.interrupts.display()
    );

    let fragment = load_fragment(chip, csv_dir, sources)?;
    write_svd(output, &fragment, version)
}

fn load_fragment(chip: &str, csv_dir: &Path, sources: &GenerateSources) -> Result<RegdescFragment> {
    let mut instances = idf::load_reg_base_h(&sources.reg_base)?;
    let interrupts = idf::load_interrupts_h(&sources.interrupts, chip)?;
    log::info!(
        "loaded {} interrupts from {}",
        interrupts.len(),
        sources.interrupts.display()
    );

    let mut csv_overrides = HashMap::new();
    for &(csv, peri) in idf::csv_type_overrides() {
        csv_overrides.insert(csv.to_owned(), peri.to_owned());
    }

    let descriptions = load_descriptions(&peripheral_descriptions_path())?;
    config::apply_instance_descriptions(&mut instances, &descriptions);

    let required_types: HashSet<&str> = instances.iter().map(|i| i.peripheral.as_str()).collect();

    let mut peripherals = Vec::new();
    for csv_path in discover_csv_files(csv_dir)? {
        let csv_file = csv_path
            .file_name()
            .and_then(|n| n.to_str())
            .with_context(|| format!("invalid CSV path {}", csv_path.display()))?
            .to_owned();

        let content = fs::read_to_string(&csv_path)
            .with_context(|| format!("reading CSV {}", csv_path.display()))?;

        let name_override = csv_overrides.get(&csv_file).map(String::as_str);
        let probed = csv::probe_peripheral_name_from_content(&content)?;

        let peripheral_name = match name_override {
            Some(name) => name,
            None => match probed.as_deref() {
                Some(name) if required_types.contains(name) => name,
                _ => {
                    log::debug!("skipping {csv_file} (not referenced by peripheral instances)");
                    continue;
                }
            },
        };

        if !required_types.contains(peripheral_name) {
            log::warn!(
                "CSV {csv_file} maps to '{peripheral_name}' but no instance uses that type; skipping"
            );
            continue;
        }

        let mut peripheral = csv::read_peripheral_csv(&content, Some(peripheral_name))
            .with_context(|| format!("parsing CSV {}", csv_path.display()))?;

        for err in peripheral.merge_registers_fields() {
            log::warn!("merge warning in {csv_file}: {}", err.0);
        }

        log::info!("parsed {csv_file} -> {}", peripheral.name);
        peripherals.push(peripheral);
    }

    for (csv_file, peri_type) in &csv_overrides {
        if required_types.contains(peri_type.as_str())
            && !peripherals.iter().any(|p| p.name == *peri_type)
        {
            log::warn!(
                "CSV override '{csv_file}' → '{peri_type}' was not found in {}",
                csv_dir.display()
            );
        }
    }

    Ok(config::build_fragment(
        chip,
        peripherals,
        instances,
        interrupts,
    ))
}

fn write_svd(output: &Path, fragment: &RegdescFragment, version: u32) -> Result<()> {
    let xml = svd::write_svd(fragment, version);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, xml).with_context(|| format!("writing SVD to {}", output.display()))?;

    log::info!("Wrote {}", output.display());
    Ok(())
}
