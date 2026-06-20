# Embedded Camsense-X1

[crates-badge]: https://img.shields.io/crates/v/embedded-camsense-x1.svg
[crates-url]: https://crates.io/crates/embedded-camsense-x1
[docs-badge]: https://docs.rs/embedded-camsense-x1/badge.svg
[docs-url]: https://docs.rs/embedded-camsense-x1
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?labelColor=1C2C2E&style=flat-square
[ci-badge]: https://github.com/AnesBenmerzoug/embedded-camsense-x1/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/AnesBenmerzoug/embedded-camsense-x1/actions?query=workflow%3ACI+branch%3Amain

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
![MIT/Apache-2.0 licensed][license-badge]
[![Build Status][ci-badge]][ci-url]

> Platform agnostic Rust driver for the [Camsense-X1] LiDAR sensor, based on the [embedded-hal] traits.

[Camsense-X1]: https://www.camsense.cn/en/robot/camsenseX1.html
[embedded-hal]: https://github.com/rust-embedded/embedded-hal

<figure style="align-items: center; text-align: center;">
    <img alt="Picture of Camsense-X1" src="https://raw.githubusercontent.com/AnesBenmerzoug/embedded-camsense-x1/main/camsense_x1.jpg"/>
</figure>

This library provides a `no_std` interface for interacting with the [Camsense-X1] LiDAR sensor.

Most of the information about the inner workings of the LiDAR was taken from:

- [Official C++ SDK](https://github.com/camsense/SDK_V3.0/tree/master)
- [Vidicon's reverse engineering of Camsense-X1](https://github.com/Vidicon/camsense-X1)

## Features

- **`no_std` & zero-allocations**: Fully compatible with bare-metal and RTOS targets. Uses fixed-size arrays and avoids heap allocation.
- **Platform-Agnostic I/O**: Built on `embedded-hal` and `embedded-io` traits, supporting any UART peripheral and delay backend.
- **Blocking and Asynchronous Interfaces**: Provides interfaces for both blocking and asynchronous (`async` feature) communication.
- **Robust Protocol Synchronization**: Const-generic state machine for byte-level frame detection with automatic recovery from noisy UART lines.
- **Checksum Validation**: Implements exact SDK algorithm (linear accumulator with 15-bit folding) to silently drop corrupted frames.

## Device

The Camsense-X1 is a cost-effective consumer-level LiDAR with the following characteristics[^1]:

| Description         | Parameter value             |
|---------------------|-----------------------------|
| Operating range     | 100mm - 8000mm              |
| Repeatability       | <0.5%                       |
| Absolute accuracy   | <2%                         |
| Angle resolution    | 0.9°                        |
| Rotating speed      | 312±10 RPM                  |
| Power consumption   | <2W                         |
| Laser light source  | 780nm EEL/808nm VCSEL laser |
| Laser safety level  | IEC60825 Class I            |

[^1]: [Camsense-X1 Product Page](https://www.camsense.cn/en/robot/camsenseX1.html)

## Examples

See the [examples](examples/) directory for examples of using the Camsense-X1 with different micro-controllers and boards.

## Feature Flags

- `defmt` - Enable logging output using [`defmt`](https://crates.io/crates/defmt) and implement `defmt::Format` on certain types.
- `async` - Enable asynchronous interface.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
