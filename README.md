# esp-pacs

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/esp-rs/esp-pacs/CI?label=CI&logo=github&style=flat-square)
![MIT/Apache-2.0 licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)
[![Matrix](https://img.shields.io/matrix/esp-rs:matrix.org?label=join%20matrix&color=BEC5C9&logo=matrix&style=flat-square)](https://matrix.to/#/#esp-rs:matrix.org)

Peripheral Access Crates for Espressif SoCs and modules. For information on how to use these crates, please refer to the [svd2rust documentation].

If you discover any errors in an SVD file, please report the issue(s) upstream in the [espressif/svd] repository.

Please note that in order to build the PACs for Xtensa devices (**ESP32**, **ESP32-S2**, **ESP32-S3**, **ESP8266**) you will need to first install the [Rust compiler fork] with Xtensa support. The minimum supported Rust version (MSRV) for these devices is `1.58.0`.

[svd2rust documentation]: https://docs.rs/svd2rust/latest/svd2rust/
[espressif/svd]: https://github.com/espressif/svd/
[rust compiler fork]: https://github.com/esp-rs/rust-build/

## Generating the PACs

We use the workflow described by [cargo-xtask] to automate tasks within this monorepo. Currently there is only a single task which generates a Peripheral Access Crate (PAC) for the specified chip(s). We've opted not to use the `cargo xtask` alias, as this requires a workspace which can cause problems when using different toolchains and targets like we are.

```text
xtask

USAGE:
    xtask [OPTIONS] [CHIPS]...

ARGS:
    <CHIPS>...    Chip(s) to target [possible values: esp32, esp32c3, esp32s2, esp32s3, esp8266]

OPTIONS:
        --generate-only    Patch the SVD and generate the PAC, but do not build it
    -h, --help             Print help information
        --patch-only       Only patch the SVD, do not generate or build the PAC
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
