# Changelog

## [0.1.0] - 2026-06-20

[0.1.0]: https://github.com/AnesBenmerzoug/embedded-tfluna/releases/tag/v0.1.0

_This is the very first release of the `embedded-camsense-x1` crate._

It features:

- **`no_std` & zero-allocations**: Fully compatible with bare-metal and RTOS targets. Uses fixed-size arrays and avoids heap allocation.
- **Platform-Agnostic I/O**: Built on `embedded-hal`, `embedded-io` traits for blocking, `embedded-hal-async` and `embedded-io-async` for asynchronous communication,
  supporting any UART peripheral and delay backend.
- **Blocking and Asynchronous Interfaces**: Provides interfaces for both blocking and asynchronous (`async` feature) communication.
- **Robust Protocol Synchronization**: Const-generic state machine for byte-level frame detection with automatic recovery from noisy UART lines.
- **Checksum Validation**: Implements exact SDK algorithm (linear accumulator with 15-bit folding) to silently drop corrupted frames.


