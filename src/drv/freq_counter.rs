//! Frequency Counter.

use crate::map::periph;
use crate::reg::prelude::*;
use drone_core::{spin_until, spin_while};

/// Frequency Counter.
pub struct FreqCounter {
    periph: periph::Fc0,
}

/// Clock sent to frequency counter.
#[allow(missing_docs)]
#[repr(u8)]
pub enum Clock {
    Null = 0x00,
    PllSysClksrcPrimary = 0x01,
    PllUsbClksrcPrimary = 0x02,
    RoscClksrc = 0x03,
    RoscClksrcPh = 0x04,
    XoscClksrc = 0x05,
    ClksrcGpin0 = 0x06,
    ClksrcGpin1 = 0x07,
    ClkRef = 0x08,
    ClkSys = 0x09,
    ClkPeri = 0x0A,
    ClkUsb = 0x0B,
    ClkAdc = 0x0C,
    ClkRtc = 0x0D,
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
    #[inline]
    pub fn init_ref_khz(&self, ref_khz: u32) {
        self.periph.clocks_fc0_ref_khz.store(|r| r.write_fc0_ref_khz(ref_khz));
    }

    /// Measures a clocks frequency using the frequency counter.
    #[must_use]
    pub fn count_khz(&self, src: Clock) -> u32 {
        spin_while!(self.periph.clocks_fc0_status.load().running());
        self.periph.clocks_fc0_src.store(|r| r.write_fc0_src(src as u32));
        spin_until!(self.periph.clocks_fc0_status.load().done());
        self.periph.clocks_fc0_result.load().khz()
    }
}
