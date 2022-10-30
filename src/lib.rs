//! Raspberry Pi Pico (RP2040) support for Drone, an Embedded Operating System.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-raspberrypi-pico/0.15/)
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-raspberrypi-pico = { version = "0.15.0", features = [...] }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["drone-raspberrypi-pico/std"]
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod map;

/// Raw bindings to Raspberry Pi Pico SDK.
pub mod sdk {
    pub use drone_raspberrypi_pico_sdk::*;
}
