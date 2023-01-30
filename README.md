# esp-pacs

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/esp-rs/esp-pacs/ci.yml?label=CI&logo=github&style=flat-square)
![MIT/Apache-2.0 licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)
[![Matrix](https://img.shields.io/matrix/esp-rs:matrix.org?label=join%20matrix&color=BEC5C9&logo=matrix&style=flat-square)](https://matrix.to/#/#esp-rs:matrix.org)

Peripheral Access Crates for Espressif SoCs and modules. For information on how to use these crates, please refer to the [svd2rust documentation].

If you discover any errors in an SVD file, please report the issue(s) upstream in the [espressif/svd] repository.

[svd2rust documentation]: https://docs.rs/svd2rust/latest/svd2rust/
[espressif/svd]: https://github.com/espressif/svd/

## MSRV

The **M**inimum **S**upported **R**ust **V**ersions are:

- `1.60.0` for RISC-V devices (**ESP32-C2**, **ESP32-C3**, **ESP32-C6**, **ESP32-H2**)
- `1.60.0` for Xtensa devices (**ESP32**, **ESP32-S2**, **ESP32-S3**, **ESP8266**)

Note that targeting the Xtensa ISA currently requires the use of the [esp-rs/rust] compiler fork, which can be installed using [esp-rs/espup].

RISC-V is supported by the official Rust compiler.

[esp-rs/rust]: https://github.com/esp-rs/rust
[esp-rs/espup]: https://github.com/esp-rs/espup

## Patching the SVDs

[svdtools](https://github.com/stm32-rs/svdtools) is used to patch the SVDs rather than modifying the files directly. This makes it easier to upstream the changes to the official SVDs, which is done periodically. A full description of the patching format is available in the `svdtools` README.

We ask that you please using the YAML patching format rather than directly modifying the SVDs in any contributions.

If you submit a pull request we kindly ask that you keep the patches and generated source files in separate commits; this makes it easier for us to review the changes, which can often involve dozens or even hundreds of files.

## Generating the PACs

We use the workflow described by [cargo-xtask] to automate tasks within this monorepo. Currently there is only a single task which generates a Peripheral Access Crate (PAC) for the specified chip(s). We've opted not to use the `cargo xtask` alias, as this requires a workspace which can cause problems when using different toolchains and targets like we are.

```text
Usage: xtask [OPTIONS] [CHIPS]...

Arguments:
  [CHIPS]...  Chip(s) to target [possible values: esp32, esp32c2, esp32c3, esp32c6, esp32h2, esp32s2, esp32s3, esp8266]

Options:
  -p, --patch-only     Only patch the SVD, do not generate or build the PAC
  -g, --generate-only  Patch the SVD and generate the PAC, but do not build it
  -h, --help           Print help
```

For example, to generate a PAC for the ESP32-C3 _without_ subsequently building the crate, from within the `xtask/` directory run:

```shell
$ cargo run -- --generate-only esp32c3
```

[cargo-xtask]: https://github.com/matklad/cargo-xtask/

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
