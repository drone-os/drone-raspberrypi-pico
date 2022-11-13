//! Crystal Oscillator (XOSC).

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_xosc_inner;

    /// Crystal Oscillator peripheral.
    pub struct XoscPeriph;

    crate::map::reg;
    crate::map::periph::xosc;

    XOSC {
        CTRL;
        STATUS;
        DORMANT;
        STARTUP;
        COUNT;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_xosc {
    ($($tt:tt)*) => {
        $crate::periph_xosc_inner!($($tt)*);
    };
}

/// Extracts Crystal Oscillator register tokens.
#[doc(inline)]
pub use crate::periph_xosc;
