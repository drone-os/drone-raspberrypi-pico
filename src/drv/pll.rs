//! Phase Locked Loop control driver.

use crate::map::periph;
use crate::map::periph::pll::traits::*;
use crate::reg::prelude::*;
use crate::sdk::XOSC_MHZ;
#[cfg(not(feature = "host"))]
use crate::sdk::{PICO_PLL_VCO_MAX_FREQ_MHZ, PICO_PLL_VCO_MIN_FREQ_MHZ};
use core::ops::RangeInclusive;
use drone_core::spin_until;

/// Acceptable Voltage Controlled Oscillator (VCO) range.
#[cfg(not(feature = "host"))]
pub const VCO_FREQ_RANGE: RangeInclusive<u32> =
    PICO_PLL_VCO_MIN_FREQ_MHZ * 1_000_000..=PICO_PLL_VCO_MAX_FREQ_MHZ * 1_000_000;

/// Acceptable feedback divider range.
pub const FBDIV_RANGE: RangeInclusive<u32> = 16..=320;

/// Acceptable post divider range.
pub const POSTDIV_RANGE: RangeInclusive<u32> = 1..=7;

/// Phase Locked Loop control driver.
pub struct Pll<T: periph::PllMap> {
    periph: periph::Pll<T>,
}

impl<T: periph::PllMap> Pll<T> {
    /// Creates a new Phase Locked Loop control driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Pll<T>) -> Self {
        Self { periph }
    }

    /// Releases the Phase Locked Loop control peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Pll<T> {
        self.periph
    }

    /// Initializes the PLL.
    ///
    /// This function will block until the PLL is locked.
    ///
    /// # Panics
    ///
    /// * If `vco_freq` is out of [`VCO_FREQ_RANGE`].
    /// * If `vco_freq / (`[`XOSC_MHZ`]` * 1_000_000 / refdiv)` is out of
    ///   [`FBDIV_RANGE`].
    /// * If `postdiv1` is out of [`POSTDIV_RANGE`].
    /// * If `postdiv2` is out of [`POSTDIV_RANGE`].
    /// * If `postdiv2 > postdiv1`.
    /// * If [`XOSC_MHZ`]` / refdiv > vco_freq / 16`.
    pub fn init(&self, refdiv: u32, vco_freq: u32, postdiv1: u32, postdiv2: u32) {
        let ref_mhz = XOSC_MHZ / refdiv;
        #[cfg(not(feature = "host"))]
        assert!(VCO_FREQ_RANGE.contains(&vco_freq));
        let fbdiv = vco_freq / (ref_mhz * 1_000_000);
        assert!(FBDIV_RANGE.contains(&fbdiv));
        assert!(POSTDIV_RANGE.contains(&postdiv1));
        assert!(POSTDIV_RANGE.contains(&postdiv2));
        assert!(postdiv2 <= postdiv1);
        assert!(ref_mhz <= vco_freq / 16);
        self.periph.pll_cs.store_reg(|r, v| {
            r.refdiv().write(v, refdiv);
        });
        self.periph.pll_fbdiv_int.store_reg(|r, v| {
            r.fbdiv_int().write(v, fbdiv);
        });
        self.periph.pll_pwr.modify_clear_reg(|r, v| {
            r.pd().set(v);
            r.vcopd().set(v);
        });
        spin_until!(self.periph.pll_cs.lock().read(&self.periph.pll_cs.load_val()));
        self.periph.pll_prim.store_reg(|r, v| {
            r.postdiv1().write(v, postdiv1);
            r.postdiv2().write(v, postdiv2);
        });
        self.periph.pll_pwr.postdivpd().clear_bit_alias();
    }
}
