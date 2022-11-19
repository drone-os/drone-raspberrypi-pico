//! Pad Control - QSPI Bank.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts QSPI Bank Pad Control register tokens.
    pub macro periph_pads_qspi;

    /// QSPI Bank Pad Control peripheral.
    pub struct PadsQspi;

    crate::map::reg;
    crate::map::periph::pads_qspi;

    PADS_QSPI {
        VOLTAGE_SELECT;
    }
}
