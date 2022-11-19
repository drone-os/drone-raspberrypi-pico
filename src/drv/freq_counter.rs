//! Frequency Counter.

use crate::map::periph;
use crate::reg::prelude::*;
use drone_core::{spin_until, spin_while};

/// Frequency Counter.
pub struct FreqCounter {
    periph: periph::Fc0,
}

impl FreqCounter {
    /// Creates a new Frequency Counter driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Fc0) -> Self {
        Self { periph }
    }

    /// Releases the Frequency Counter peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Fc0 {
        self.periph
    }

    /// Initializes the reference clock frequency.
    pub fn init_ref_khz(&self, ref_khz: u32) {
        self.periph.clocks_fc0_ref_khz.store(|r| r.write_fc0_ref_khz(ref_khz));
    }

    /// Measures a clocks frequency using the frequency counter.
    #[must_use]
    pub fn count_khz(&self, src: u32) -> u32 {
        spin_while!(self.periph.clocks_fc0_status.load().running());
        self.periph.clocks_fc0_src.store(|r| r.write_fc0_src(src));
        spin_until!(self.periph.clocks_fc0_status.load().done());
        self.periph.clocks_fc0_result.load().khz()
    }
}
