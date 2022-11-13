[![crates.io](https://img.shields.io/crates/v/drone-raspberrypi-pico.svg)](https://crates.io/crates/drone-raspberrypi-pico)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# Drone STM32 Map

<!-- cargo-rdme start -->

Raspberry Pi Pico (RP2040) support for Drone, an Embedded Operating System.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-raspberrypi-pico/0.15/)

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-raspberrypi-pico = { version = "0.15.0", features = [...] }
```

Add or extend `host` feature as follows:

```toml
[features]
host = ["drone-raspberrypi-pico/host"]
```

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
