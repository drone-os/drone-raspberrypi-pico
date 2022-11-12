//! Running code on multiple cores.

#![cfg_attr(feature = "std", allow(unused_imports, unreachable_code, unused_variables))]

mod macros;

use crate::sdk;
use core::mem;

/// Launch code on core 1.
///
/// # Safety
///
/// * Core 1 must previously has been reset either as a result of a system reset
///   or by calling [`reset_core1`].
/// * `vtable` must be a valid pointer to a vector table for core 1.
pub unsafe fn launch_core1(vtable: *const usize) {
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
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
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        sdk::multicore_reset_core1();
    }
}
