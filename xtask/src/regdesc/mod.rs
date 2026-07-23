mod config;
mod csv;
mod merge;
mod model;
mod svd;
mod util;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use config::{chip_config_dir, load_descriptions, load_metadata, load_regdesc_config, peripheral_descriptions_path};
use model::RegdescFragment;

pub use config::default_output_path;

/// Main entry: load config + CSVs, build the chip model, write an SVD file.
pub fn generate_base_svd(chip: &str, csv_dir: &Path, output: &Path, version: u32) -> Result<()> {
    log::info!(
        "Generating base SVD for {chip} (version {version}) using config {}",
        chip_config_dir(chip).display()
    );

    let fragment = load_fragment(chip, csv_dir)?;
    write_svd(output, &fragment, version)
}

/// Reads all YAML config and CSV files and assembles one in-memory chip description.
fn load_fragment(chip: &str, csv_dir: &Path) -> Result<RegdescFragment> {
    let config_dir = chip_config_dir(chip);
    let csv_map = load_regdesc_config(&config_dir.join("regdesc.yml"))?;
    let (mut instances, interrupts) = load_metadata(&config_dir)?;
    let descriptions = load_descriptions(&peripheral_descriptions_path())?;

    config::apply_instance_descriptions(&mut instances, &descriptions);

    let mut csv_files: Vec<_> = csv_map.keys().cloned().collect();
    csv_files.sort();

    let mut peripherals = Vec::with_capacity(csv_files.len());
    for csv_file in csv_files {
        let options = &csv_map[&csv_file];
        let csv_path = csv_dir.join(&csv_file);
        let content = fs::read_to_string(&csv_path)
            .with_context(|| format!("reading CSV {}", csv_path.display()))?;

        let mut peripheral = csv::read_peripheral_csv(&content, options.name.as_deref())
            .with_context(|| format!("parsing CSV {}", csv_path.display()))?;

        for err in peripheral.merge_registers_fields() {
            log::warn!("merge warning in {csv_file}: {}", err.0);
        }

        log::info!("parsed {csv_file} -> {}", peripheral.name);
        peripherals.push(peripheral);
    }

    Ok(config::build_fragment(chip, peripherals, instances, interrupts))
}

/// Turns the chip model into XML and saves it (creates parent dirs if needed).
fn write_svd(output: &Path, fragment: &RegdescFragment, version: u32) -> Result<()> {
    let xml = svd::write_svd(fragment, version);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, xml).with_context(|| format!("writing SVD to {}", output.display()))?;

    log::info!("Wrote {}", output.display());
    Ok(())
}
