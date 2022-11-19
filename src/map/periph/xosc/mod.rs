//! Crystal Oscillator (XOSC).

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Crystal Oscillator register tokens.
    pub macro periph_xosc;

    /// Crystal Oscillator peripheral.
    pub struct Xosc;

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
