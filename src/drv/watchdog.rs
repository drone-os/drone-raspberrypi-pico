//! Hardware Watchdog Timer driver.

use crate::map::periph;
use crate::reg::prelude::*;
use crate::sdk::XOSC_MHZ;

/// Hardware Watchdog Timer driver.
pub struct Watchdog {
    periph: periph::Watchdog,
}

impl Watchdog {
    /// Creates a new Hardware Watchdog Timer driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Watchdog) -> Self {
        Self { periph }
    }

    /// Releases the Hardware Watchdog Timer peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Watchdog {
        self.periph
    }

    /// Starts the watchdog tick.
    ///
    /// Important: This function also provides a tick reference to the timer.
    pub fn start_tick(&self) {
        self.periph.watchdog_tick.store(|r| r.write_cycles(XOSC_MHZ).set_enable());
    }
}
