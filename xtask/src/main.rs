use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Error, Result};
use clap::{Parser, ValueEnum};
use form::util::create_directory_structure;
use log::{info, warn, LevelFilter};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use svd2rust::{generate::device::render, load_from, util::build_rs, Config, Target};
use svdtools::patch::process_file;
use toml::Value;

#[derive(Debug, Clone, Display, EnumIter, EnumString, ValueEnum)]
#[strum(serialize_all = "lowercase")]
enum Chip {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32c6,
    Esp32h2,
    Esp32s2,
    Esp32s3,
    Esp8266,
}

#[derive(Debug, Parser)]
struct Cli {
    /// Chip(s) to target
    #[clap(value_enum)]
    chips: Vec<Chip>,
    /// Only patch the SVD, do not generate or build the PAC
    #[clap(short, long, conflicts_with = "generate_only")]
    patch_only: bool,
    /// Patch the SVD and generate the PAC, but do not build it
    #[clap(short, long, conflicts_with = "patch_only")]
    generate_only: bool,
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("xtask", LevelFilter::Info)
        .init();

    let args = Cli::parse();

    // The directory containing the cargo manifest for the 'xtask' package is a
    // subdirectory within the cargo workspace.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace.parent().unwrap().canonicalize()?;

    // One or more chips can be specified as command-line arguments. If none are
    // provided then the task will be run for all chips.
    let chips = if !args.chips.is_empty() {
        args.chips.iter().map(|c| c.to_string()).collect::<Vec<_>>()
    } else {
        Chip::iter().map(|c| c.to_string()).collect::<Vec<_>>()
    };

    for chip in chips {
        info!("chip: {chip}");
        let path = workspace.join(&chip);

        // Always patch the SVD upon execution. If '--patch-only' has NOT been set, then
        // additionally generate and format the PAC. If '--generate-only' has NOT been
        // set, clean and build the PAC crate.
        patch_svd(&chip, &path)?;

        if !args.patch_only {
            // Remove the src/ directory before we generate. If this fails we will assume
            // it's because the directory does not exist.
            if fs::remove_dir_all(&path.join("src")).is_err() {
                warn!("unable to remove 'src/' directory");
            }

            generate_pac(&chip, &path)?;
            format(&path)?;

            if !args.generate_only {
                clean(&path)?;
                build(&path)?;
            }
        }

        info!("finished with {chip}")
    }

    Ok(())
}

fn patch_svd(chip: &str, path: &Path) -> Result<()> {
    let svd_path = path.join("svd");
    let yaml_file = svd_path.join("patches").join(format!("{chip}.yaml"));
    process_file(&yaml_file)?;

    let from = svd_path.join(format!("{chip}.base.svd.patched"));
    let to = svd_path.join(format!("{chip}.svd"));
    info!("applying patches to SVD file");
    fs::rename(from, to)?;

    Ok(())
}

fn generate_pac(chip: &str, path: &Path) -> Result<()> {
    let svd_file = path.join("svd").join(format!("{chip}.svd"));
    info!("generating PAC from '{}'", svd_file.display());

    let target = if get_build_target(path)?.contains("riscv") {
        Target::RISCV
    } else {
        Target::XtensaLX
    };

    let config = Config {
        target,
        output_dir: path.to_path_buf(),
        const_generic: true,

        ..match target {
            Target::RISCV => Config {
                interrupt_link_section: Some(".trap.rodata".to_owned()),
                ..Config::default()
            },
            _ => Config::default(),
        }
    };

    let input = fs::read_to_string(svd_file)?;
    let device = load_from(&input, &config)?;

    let mut device_x = String::new();
    let items = render(&device, &config, &mut device_x)?;
    let data = items.to_string();

    // Here we will sneakily patch in our own logo for the documentation :)
    let data = data.replace(
        "# ! [no_std]",
        "# ! [doc(html_logo_url = \"https://avatars.githubusercontent.com/u/46717278\")]\n# ! [no_std]",
    );

    let mut file = File::create(path.join("lib.rs"))?;
    file.write_all(data.as_ref())?;

    writeln!(File::create(path.join("device.x"))?, "{}", device_x)?;
    writeln!(File::create(path.join("build.rs"))?, "{}", build_rs())?;

    Ok(())
}

fn format(path: &PathBuf) -> Result<()> {
    info!("running `form` and `rustfmt` on PAC");
    let lib_file = path.join("lib.rs");

    let base_dir = path.join("src");
    let string_contents = fs::read_to_string(&lib_file)?;
    create_directory_structure(base_dir, &string_contents).map_err(Error::msg)?;

    fs::remove_file(&lib_file)?;

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
    let channel = get_release_channel(path)?;
    let target = get_build_target(path)?;

    info!("building PAC using '{channel}' channel and targeting '{target}'");
    Command::new("cargo")
        .args([
            &format!("+{channel}"),
            "build",
            "-Z",
            "build-std=core",
            "--target",
            &target,
        ])
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

fn get_release_channel(path: &Path) -> Result<String> {
    let toolchain_file = path.join("rust-toolchain.toml");
    let channel = extract_toml_value(&toolchain_file, &["toolchain", "channel"])?;

    Ok(channel)
}

fn get_build_target(path: &Path) -> Result<String> {
    let config_file = path.join(".cargo").join("config.toml");
    let target = extract_toml_value(&config_file, &["build", "target"])?;

    Ok(target)
}

fn extract_toml_value(path: &Path, keys: &[&str]) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let value = contents.parse::<Value>()?;

    let mut item = &value;
    for key in keys {
        item = item.get(key).unwrap();
    }

    let item = item.to_string().replace('\"', "");

    Ok(item)
}
