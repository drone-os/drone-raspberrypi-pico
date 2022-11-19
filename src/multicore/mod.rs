//! Running code on multiple cores.

#![cfg_attr(feature = "host", allow(unused_imports, unreachable_code, unused_variables))]

mod macros;

use crate::sdk;
use core::mem;
use drone_core::reg::prelude::*;
use drone_core::token::Token;
use drone_raspberrypi_pico_map_pieces::reg::sio::Cpuid;

/// Returns 0 when called on processor core 0, and 1 when called on processor
/// core 1.
#[must_use]
pub fn cpuid() -> u32 {
    // Safe because the register is read-only.
    let cpuid = unsafe { Cpuid::<Urt>::take() };
    cpuid.load_bits()
}

/// Launch code on core 1.
///
/// # Safety
///
/// * Core 1 must previously has been reset either as a result of a system reset
///   or by calling [`reset_core1`].
/// * `vtable` must be a valid pointer to a vector table for core 1.
pub unsafe fn launch_core1(vtable: *const usize) {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        sdk::multicore_launch_core1_raw(
            mem::transmute(vtable.add(1).read_volatile()),
            vtable.read_volatile() as _,
            vtable as _,
        );
    }
}

/// Resets core 1 into its initial state (ready for launching code against via
/// [`launch_core1`] and similar methods)
///
/// # Safety
///
/// * Should only be called from core 0.
pub unsafe fn reset_core1() {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        sdk::multicore_reset_core1();
    }
}
