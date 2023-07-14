# esp-pacs

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/esp-rs/esp-pacs/ci.yml?label=CI&logo=github&style=flat-square)
![MIT/Apache-2.0 licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)
[![Matrix](https://img.shields.io/matrix/esp-rs:matrix.org?label=join%20matrix&color=BEC5C9&logo=matrix&style=flat-square)](https://matrix.to/#/#esp-rs:matrix.org)

**P**eripheral **A**ccess **C**rates (PACs) for Espressif SoCs and modules.

For information on how to use these crates, please refer to the [svd2rust documentation].

[svd2rust documentation]: https://docs.rs/svd2rust/latest/svd2rust/

## Getting Started

We ask that you please read each of the below sections prior to contributing to this repository.

### xtask

The `xtask` package is a binary application, based on the workflow outlined by [cargo-xtask], which provides some simple utilities which make it easier to work with this repository. It is primarily used for patching SVDs and re-generating the PACs, but also provides some other functionality.

```text
Usage: xtask <COMMAND>

Commands:
  patch         Patch the specified package(s)'s SVD file
  generate      Generate the specified package(s)
  build         Build the specified package(s)
  bump-version  Bump the version of the specified package(s)
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

This application can be found in the `xtask/` directory in the root of the repository.

[cargo-xtask]: https://github.com/matklad/cargo-xtask/

### Patching the SVDs

[svdtools] is used to patch the SVDs rather than modifying the files directly. This makes it easier to upstream the changes to the official SVDs, which is done periodically, and then subsequently update the base SVDs here. A full description of the patching format is available in the [svdtools] README.

Patching is accomplished using the `xtask` package's `patch` subcommand. The `patch` subcommand accepts zero or more arguments:

- If zero arguments are provided, it will patch all PACs in the repository
- If one or more arguments are provided, it will patch each specified PAC

In order to apply patches to one or more chips' SVDs, from within the `xtask/` directory you can run:

```bash
# Only patch the ESP32-C3's SVD:
cargo run -- patch esp32c3
# Patch the SVDs for the ESP32, ESP32-S2, and ESP32-S3:
cargo run -- patch esp32 esp32s2 esp32s3
# Patch all the SVDs!
cargo run -- patch
```

### Generating the PACs

PACs are also generated using an `xtask` subcommand, this time the `generate` subcommand. Again, the `generate` subcommand accepts zero or more arguments:

- If zero arguments are provided, it will patch and (re-)generate all PACs in the repository
- If one or more arguments are provided, it will patch and (re-)generate each specified PAC

In order to apply re-generate to one or more chips'SVDs, from within the `xtask/` directory you can run:

```shell
# Only generate the ESP32-C3's PAC:
cargo run -- generate esp32c3
# Generate the PACs for the ESP32, ESP32-S2, and ESP32-S3:
cargo run -- generate esp32 esp32s2 esp32s3
# Generate all the PACs!
cargo run -- generate
```

**Note:** It's also possible to build the PACs after patching and generating them, using the `build` subcommand instead of `generate`. This otherwise works the same as the process described above, but confirms that all generate source files build successfully. It's a good idea to do this prior to opening a pull request.

[svdtools]: https://github.com/stm32-rs/svdtools
[cargo-xtask]: https://github.com/matklad/cargo-xtask/

### Opening a Pull Request

Prior to opening a pull request, we ask that you please:

- Ensure that **all** required files have been commited
  - This include patch files (YAML), Rust source files, and (only sometimes) the `device.x` linker script
  - Please do not force add files to your commits; if a file is ignored in `.gitignore` it should not be included!
- Ensure that you are able to patch, generate, and build each PAC using the `xtask` package

## MSRV

The **M**inimum **S**upported **R**ust **V**ersions are:

- `1.65.0` for RISC-V devices (**ESP32-C2/C3/C6**, **ESP32-H2**, **ESP32-S2/S3 RISC-V ULP coprocessors**)
- `1.65.0` for Xtensa devices (**ESP32**, **ESP32-S2/S3**, **ESP8266**)

Note that targeting the Xtensa ISA currently requires the use of the [esp-rs/rust] compiler fork, which can be installed using [esp-rs/espup].

RISC-V is supported by the official Rust compiler.

[esp-rs/rust]: https://github.com/esp-rs/rust
[esp-rs/espup]: https://github.com/esp-rs/espup

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
