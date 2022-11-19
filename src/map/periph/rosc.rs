//! Ring Oscillator (ROSC).

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Ring Oscillator register tokens.
    pub macro periph_rosc;

    /// Ring Oscillator peripheral.
    pub struct Rosc;

    crate::map::reg;
    crate::map::periph::rosc;

    ROSC {
        CTRL;
        FREQA;
        FREQB;
        DORMANT;
        DIV;
        PHASE;
        STATUS;
        RANDOMBIT;
        COUNT;
    }
}
