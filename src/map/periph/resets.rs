//! Resets.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Resets register tokens.
    pub macro periph_resets;

    /// Resets peripheral.
    pub struct Resets;

    crate::map::reg;
    crate::map::periph::resets;

    RESETS {
        RESET;
        WDSEL;
        RESET_DONE;
    }
}
