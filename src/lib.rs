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

use core::cell::UnsafeCell;

pub mod map;
pub mod reg;

/// Raw bindings to Raspberry Pi Pico SDK.
pub mod sdk {
    pub use drone_raspberrypi_pico_sdk::*;
}

/// Initializes Raspberry Pi Pico SDK runtime.
///
/// # Safety
///
/// This function is not thread-safe.
pub unsafe fn init() {
    type InitFn = extern "C" fn();
    extern "C" {
        static PREINIT_ARRAY_BASE: UnsafeCell<InitFn>;
        static PREINIT_ARRAY_END: UnsafeCell<InitFn>;
    }
    unsafe {
        let mut ptr = PREINIT_ARRAY_BASE.get();
        while ptr < PREINIT_ARRAY_END.get() {
            (*ptr)();
            ptr = ptr.add(1);
        }
    }
}
