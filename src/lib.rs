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
#![allow(clippy::doc_markdown)]
#![cfg_attr(not(feature = "host"), no_std)]

use core::cell::UnsafeCell;
use drone_core::reg::prelude::*;
use drone_core::token::Token;
use drone_raspberrypi_pico_map_pieces::reg::sio::Cpuid;

pub mod map;
pub mod multicore;
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

/// Returns 0 when called on processor core 0, and 1 when called on processor
/// core 1.
#[must_use]
pub fn cpuid() -> u32 {
    // Safe because the register is read-only.
    let cpuid = unsafe { Cpuid::<Urt>::take() };
    cpuid.load_bits()
}
