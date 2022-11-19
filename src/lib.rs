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
//! Add or extend `host` feature as follows:
//!
//! ```toml
//! [features]
//! host = ["drone-raspberrypi-pico/host"]
//! ```

#![warn(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]
#![feature(marker_trait_attr)]
#![allow(clippy::doc_markdown, clippy::wildcard_imports)]
#![cfg_attr(not(feature = "host"), no_std)]

pub mod drv;
pub mod map;
pub mod multicore;
pub mod reg;
pub mod thr;

/// Raw bindings to Raspberry Pi Pico SDK.
pub mod sdk {
    pub use drone_raspberrypi_pico_sdk::*;
}

use core::cell::UnsafeCell;

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
