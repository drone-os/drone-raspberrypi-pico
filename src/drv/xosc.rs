//! Crystal Oscillator (XOSC) driver.

use crate::map::periph;
use crate::reg::prelude::*;
use crate::sdk::XOSC_MHZ;
use drone_core::spin_until;

const STARTUP_DELAY: u32 = (XOSC_MHZ * 1000 + 128) / 256;

/// Crystal Oscillator (XOSC) driver.
pub struct Xosc {
    periph: periph::Xosc,
}

impl Xosc {
    /// Creates a new Crystal Oscillator (XOSC) driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Xosc) -> Self {
        Self { periph }
    }

    /// Releases the Crystal Oscillator (XOSC) peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Xosc {
        self.periph
    }

    /// Initializes the crystal oscillator system.
    ///
    /// This function will block until the crystal oscillator has stabilised.
    pub fn init(&self) {
        self.periph.xosc_ctrl.store(|r| r.write_freq_range(0xAA0)); // 1-15 MHz
        self.periph.xosc_startup.store(|r| r.write_delay(STARTUP_DELAY));
        self.periph.xosc_ctrl.enable.set_bits_alias(0xFAB); // ENABLE bit sequence
        spin_until!(self.periph.xosc_status.load().stable());
    }
}
