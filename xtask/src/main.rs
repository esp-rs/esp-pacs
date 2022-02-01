// TODO: add some logging/output of some sort

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Error, Result};
use form::util::create_directory_structure;
use svd2rust::{generate::device::render, load_from, util::build_rs, Config, Target};
use svdtools::patch::process_file;
use toml::Value;

fn main() -> Result<()> {
    // The directory containing the cargo manifest for the 'xtask' package is a
    // subdirectory within the cargo workspace.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace.parent().unwrap().canonicalize()?;

    // Verify that a chip was specified and that it is valid, and if so perform all
    // steps necessary to generate the PAC for said chip.
    if let Some(chip) = env::args().nth(1) {
        let chip = chip.to_lowercase().replace("-", "");
        let chip = chip.as_str();

        match chip {
            "esp32" | "esp32c3" | "esp32s2" | "esp32s3" => {
                let path = workspace.join(chip);

                // Remove the src/ directory before we begin. If this fails we will assume it's
                // because the directory does not exist.
                fs::remove_dir_all(&path.join("src")).ok();

                patch_svd(chip, &path)?;
                generate_pac(chip, &path)?;
                format(&path)?;
                clean(&path)?;
                build(&path)?;
            }
            _ => bail!("invalid chip '{}' specified", chip),
        }
    } else {
        bail!("no chip specified");
    }

    Ok(())
}

fn patch_svd(chip: &str, path: &PathBuf) -> Result<()> {
    let svd_path = path.join("svd");

    let yaml_file = svd_path.join("patches").join(format!("{chip}.yaml"));
    process_file(&yaml_file)?;

    let from = svd_path.join(format!("{chip}.base.svd.patched"));
    let to = svd_path.join(format!("{chip}.svd"));
    fs::rename(from, to)?;

    Ok(())
}

fn generate_pac(chip: &str, path: &PathBuf) -> Result<()> {
    let svd_file = path.join("svd").join(format!("{chip}.svd"));
    let input = fs::read_to_string(svd_file)?;

    let mut config = Config::default();
    config.target = Target::XtensaLX;
    config.output_dir = path.clone();

    let device = load_from(&input, &config)?;

    let mut device_x = String::new();
    let items = render(&device, &config, &mut device_x)?;
    let data = items.to_string().replace("]", "]\n");

    let mut file = File::create(path.join("lib.rs"))?;
    file.write_all(data.as_ref())?;

    writeln!(File::create(path.join("device.x"))?, "{}", device_x)?;
    writeln!(File::create(path.join("build.rs"))?, "{}", build_rs())?;

    Ok(())
}

fn format(path: &PathBuf) -> Result<()> {
    let lib_file = path.join("lib.rs");

    let base_dir = path.join("src");
    let string_contents = fs::read_to_string(&lib_file)?;
    create_directory_structure(base_dir, string_contents).map_err(Error::msg)?;

    fs::remove_file(&lib_file)?;

    // TODO: consider using 'rustfmt' directly, as a library
    Command::new("cargo")
        .arg("fmt")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn clean(path: &PathBuf) -> Result<()> {
    Command::new("cargo")
        .arg("clean")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn build(path: &PathBuf) -> Result<()> {
    let channel = get_channel(path)?;
    let target = get_target(path)?;

    Command::new("cargo")
        .args(&[
            &format!("+{channel}"),
            "build",
            "-Z",
            "build-std",
            "--target",
            &target,
        ])
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

fn get_channel(path: &PathBuf) -> Result<String> {
    let toolchain_file = path.join("rust-toolchain.toml");
    let channel = extract_toml_value(&toolchain_file, &["toolchain", "channel"])?;

    Ok(channel)
}

fn get_target(path: &PathBuf) -> Result<String> {
    let config_file = path.join(".cargo").join("config.toml");
    let target = extract_toml_value(&config_file, &["build", "target"])?;

    Ok(target)
}

fn extract_toml_value(path: &PathBuf, keys: &[&str]) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let value = contents.parse::<Value>()?;

    let mut item = &value;
    for key in keys {
        item = item.get(key).unwrap();
    }

    let item = item.to_string().replace("\"", "");

    Ok(item)
}
