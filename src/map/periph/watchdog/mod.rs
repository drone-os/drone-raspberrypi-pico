//! Watchdog.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_watchdog_inner;

    /// Watchdog peripheral.
    pub struct WatchdogPeriph;

    crate::map::reg;
    crate::map::periph::watchdog;

    WATCHDOG {
        CTRL;
        LOAD;
        REASON;
        SCRATCH0;
        SCRATCH1;
        SCRATCH2;
        SCRATCH3;
        SCRATCH4;
        SCRATCH5;
        SCRATCH6;
        SCRATCH7;
        TICK;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_watchdog {
    ($($tt:tt)*) => {
        $crate::periph_watchdog_inner!($($tt)*);
    };
}

/// Extracts Crystal Oscillator register tokens.
#[doc(inline)]
pub use crate::periph_watchdog;
