# esp32p4-lp

[![Crates.io](https://img.shields.io/crates/v/esp32p4-lp?labelColor=1C2C2E&color=C96329&logo=Rust&style=flat-square)](https://crates.io/crates/esp32p4-lp)
[![docs.rs](https://img.shields.io/docsrs/esp32p4-lp?labelColor=1C2C2E&color=C96329&logo=rust&style=flat-square)](https://docs.rs/esp32p4-lp)
![Crates.io](https://img.shields.io/crates/l/esp32p4-lp?labelColor=1C2C2E&style=flat-square)
[![Matrix](https://img.shields.io/matrix/esp-rs:matrix.org?label=join%20matrix&labelColor=1C2C2E&color=BEC5C9&logo=matrix&style=flat-square)](https://matrix.to/#/#esp-rs:matrix.org)

A **P**eripheral **A**ccess **C**rate (**PAC**) for the **ESP32-P4's LP coprocessor** from Espressif. See the [svd2rust] repository for more information on how to use this crate.

> **Silicon revision:** This crate targets ESP32-P4 silicon revision 3.0 or later (ECO5 onward). Earlier revisions are not supported.

If you find any problems with the included SVD file please open an issue in the [espressif/svd] repository so that the fixes can be applied upstream.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[espressif/svd]: https://github.com/espressif/svd

## [Documentation](https://docs.rs/esp32p4-lp)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
