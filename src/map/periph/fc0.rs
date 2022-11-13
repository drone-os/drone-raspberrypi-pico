//! Frequency Counter.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_fc0_inner;

    /// Frequency Counter.
    pub struct Fc0Periph;

    crate::map::reg;
    crate::map::periph::fc0;

    CLOCKS {
        FC0_REF_KHZ;
        FC0_MIN_KHZ;
        FC0_MAX_KHZ;
        FC0_DELAY;
        FC0_INTERVAL;
        FC0_SRC;
        FC0_STATUS;
        FC0_RESULT;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_fc0 {
    ($($tt:tt)*) => {
        $crate::periph_fc0_inner!($($tt)*);
    };
}

/// Extracts Frequency Counter register tokens.
#[doc(inline)]
pub use crate::periph_fc0;
