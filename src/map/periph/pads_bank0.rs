//! Pad Control - User Bank.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts User Bank Pad Control register tokens.
    pub macro periph_pads_bank0;

    /// User Bank Pad Control peripheral.
    pub struct PadsBank0;

    crate::map::reg;
    crate::map::periph::pads_bank0;

    PADS_BANK0 {
        VOLTAGE_SELECT;
        SWCLK;
        SWD;
    }
}
