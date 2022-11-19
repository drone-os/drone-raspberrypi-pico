//! Hardware Reset driver.

use crate::map::{periph, reg};
use crate::reg::prelude::*;
use drone_core::bitfield::Bitfield;
use drone_core::spin_until;

/// Hardware Reset driver.
pub struct Resets {
    periph: periph::Resets,
}

impl Resets {
    /// Creates a new Hardware Reset driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Resets) -> Self {
        Self { periph }
    }

    /// Releases the Hardware Reset peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Resets {
        self.periph
    }

    /// Brings specified HW blocks out of reset and wait for completion.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use drone_raspberrypi_pico::map::rp2040_reg_tokens;
    /// # rp2040_reg_tokens! { index => pub Regs; }
    /// # let reg = unsafe { <Regs as drone_core::token::Token>::take() };
    /// use drone_raspberrypi_pico::drv::Resets;
    /// use drone_raspberrypi_pico::periph_resets;
    ///
    /// let resets = Resets::new(periph_resets!(reg));
    /// resets.unreset(|r| r.set_pll_sys().set_pads_bank0().set_io_bank0());
    /// ```
    pub fn unreset(
        &self,
        f: impl for<'a> FnOnce(
            &'a mut reg::resets::reset::Hold<'a, Srt>,
        ) -> &'a mut reg::resets::reset::Hold<'a, Srt>,
    ) {
        let resets = f(&mut self.periph.resets_reset.zeroed()).val().bits();
        self.periph.resets_reset.modify_clear_bits(resets);
        spin_until!(!self.periph.resets_reset_done.load_bits() & resets == 0);
    }
}
