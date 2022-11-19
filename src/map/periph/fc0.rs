//! Frequency Counter.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Frequency Counter register tokens.
    pub macro periph_fc0;

    /// Frequency Counter.
    pub struct Fc0;

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
