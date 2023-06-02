use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Error, Result};
use clap::{Parser, Subcommand, ValueEnum};
use strum::{Display, EnumIter, IntoEnumIterator};
use svd2rust::{Config, Target};
use toml_edit::Document;

#[derive(Debug, Clone, Display, EnumIter, ValueEnum)]
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

#[derive(Debug, Clone, Copy, Display, ValueEnum)]
#[strum(serialize_all = "lowercase")]
enum Version {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Patch the specified package(s)'s SVD file
    Patch {
        /// Chip(s) to target
        #[clap(value_enum, default_values_t = Chip::iter())]
        chips: Vec<Chip>,
    },

    /// Generate the specified package(s)
    ///
    /// Additionally patches the releavant SVD(s) prior to generating the
    /// package(s).
    Generate {
        /// Chip(s) to target
        #[clap(value_enum, default_values_t = Chip::iter())]
        chips: Vec<Chip>,
    },

    /// Build the specified package(s)
    ///
    /// Additionally patches the relevant SVD(s) and generates the relevant
    /// package(s) prior to building the package(s).
    Build {
        /// Chip(s) to target
        #[clap(value_enum, default_values_t = Chip::iter())]
        chips: Vec<Chip>,
    },

    /// Bump the version of the specified package(s)
    BumpVersion {
        /// How much to bump the version
        #[clap(value_enum)]
        amount: Version,
        /// Chip(s) to target
        #[clap(value_enum, default_values_t = Chip::iter())]
        chips: Vec<Chip>,
    },
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("xtask", log::LevelFilter::Info)
        .init();

    // The directory containing the cargo manifest for the 'xtask' package is a
    // subdirectory within the cargo workspace.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace.parent().unwrap().canonicalize()?;

    match Cli::parse().command {
        Commands::Patch { chips } => chips
            .iter()
            .try_for_each(|chip| patch_svd(&workspace, chip)),
        Commands::Generate { chips } => chips
            .iter()
            .try_for_each(|chip: &Chip| generate_package(&workspace, chip)),
        Commands::Build { chips } => chips
            .iter()
            .try_for_each(|chip| build_package(&workspace, chip)),
        Commands::BumpVersion {
            chips,
            amount: version,
        } => chips
            .iter()
            .try_for_each(|chip| bump_version(&workspace, chip, version)),
    }
}

fn patch_svd(workspace: &Path, chip: &Chip) -> Result<()> {
    log::info!("applying patches to SVD file for {chip}");

    let svd_path = workspace.join(chip.to_string()).join("svd");
    let yaml_file = svd_path.join("patches").join(format!("{chip}.yaml"));
    svdtools::patch::process_file(&yaml_file)?;

    let from = svd_path.join(format!("{chip}.base.svd.patched"));
    let to = svd_path.join(format!("{chip}.svd"));
    fs::rename(from, to)?;

    Ok(())
}

fn generate_package(workspace: &Path, chip: &Chip) -> Result<()> {
    // Patch the SVD prior to generating the package:
    patch_svd(workspace, chip)?;

    let path = workspace.join(chip.to_string());

    // Remove the src/ directory before we generate. If this fails we will assume
    // it's because the directory does not exist.
    if fs::remove_dir_all(path.join("src")).is_err() {
        log::warn!("unable to remove 'src/' directory");
    }

    let svd_file = path.join("svd").join(format!("{chip}.svd"));
    log::info!("generating PAC from '{}'", svd_file.display());

    let target = if build_target(&path)?.contains("riscv") {
        Target::RISCV
    } else {
        Target::XtensaLX
    };

    let config = Config {
        target,
        output_dir: path.clone(),
        const_generic: true,
        impl_debug: true,
        impl_debug_feature: Some("impl-register-debug".to_owned()),

        ..match target {
            Target::RISCV => Config {
                interrupt_link_section: Some(".trap.rodata".to_owned()),
                ..Config::default()
            },
            _ => Config::default(),
        }
    };

    let input = fs::read_to_string(svd_file)?;
    let device = svd2rust::load_from(&input, &config)?;

    let mut device_x = String::new();
    let items = svd2rust::generate::device::render(&device, &config, &mut device_x)?;
    let data = items.to_string();

    // Here we will sneakily patch in our own logo for the documentation :)
    let data = data.replace(
        "# ! [no_std]",
        "# ! [doc(html_logo_url = \"https://avatars.githubusercontent.com/u/46717278\")]\n# ! [no_std]",
    );

    let mut file = File::create(path.join("lib.rs"))?;
    file.write_all(data.as_ref())?;

    writeln!(File::create(path.join("device.x"))?, "{}", device_x)?;
    writeln!(
        File::create(path.join("build.rs"))?,
        "{}",
        svd2rust::util::build_rs()
    )?;

    format(&path)?;

    Ok(())
}

fn build_package(workspace: &Path, chip: &Chip) -> Result<()> {
    // Patch the SVD and generate the package prior to building:
    generate_package(workspace, chip)?;

    let path = workspace.join(chip.to_string());
    clean(&path)?;

    let channel = release_channel(&path)?;
    let target = build_target(&path)?;

    log::info!("building PAC using '{channel}' channel and targeting '{target}'");
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

fn bump_version(workspace: &Path, chip: &Chip, amount: Version) -> Result<()> {
    let path = workspace.join(chip.to_string());

    let manifest_path = path.join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let mut manifest = manifest.parse::<Document>()?;

    let version = manifest["package"]["version"]
        .to_string()
        .trim()
        .trim_matches('"')
        .to_string();
    let prev_version = &version;

    let mut version = semver::Version::parse(&version)?;
    match amount {
        Version::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        Version::Minor => {
            version.minor += 1;
            version.patch = 0;
        }
        Version::Patch => {
            version.patch += 1;
        }
    }

    log::info!("bumping version for package: {chip} ({prev_version} -> {version})");
    manifest["package"]["version"] = toml_edit::value(version.to_string());
    fs::write(manifest_path, manifest.to_string())?;

    Ok(())
}

fn build_target(path: &Path) -> Result<String> {
    let config_file = path.join(".cargo").join("config.toml");
    let target = extract_toml_value(&config_file, &["build", "target"])?;

    Ok(target)
}

fn release_channel(path: &Path) -> Result<String> {
    let toolchain_file = path.join("rust-toolchain.toml");
    let channel = extract_toml_value(&toolchain_file, &["toolchain", "channel"])?;

    Ok(channel)
}

fn extract_toml_value(path: &Path, keys: &[&str]) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let document = contents.parse::<Document>()?;

    let mut item = document.as_item();
    for key in keys {
        item = item.as_table().unwrap().get(key).unwrap();
    }

    let item = item.to_string().trim().replace('"', "");

    Ok(item)
}

fn format(path: &Path) -> Result<()> {
    log::info!("running `form` and `rustfmt` on PAC");
    let lib_file = path.join("lib.rs");

    let base_dir = path.join("src");
    let string_contents = fs::read_to_string(&lib_file)?;
    form::create_directory_structure(base_dir, &string_contents).map_err(Error::msg)?;

    fs::remove_file(&lib_file)?;

    Command::new("cargo")
        .arg("fmt")
        .current_dir(path)
        .output()?;

    Ok(())
}

fn clean(path: &Path) -> Result<()> {
    Command::new("cargo")
        .arg("clean")
        .current_dir(path)
        .output()?;

    Ok(())
}
