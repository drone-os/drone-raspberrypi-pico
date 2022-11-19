//! Clock Management driver.

use crate::map::periph;
use crate::map::periph::clock::traits::*;
use crate::reg::prelude::*;
use drone_core::spin_while;

/// Clock Management driver.
pub struct Clock<T: periph::ClockMap> {
    periph: periph::Clock<T>,
}

impl<T: periph::ClockMap> Clock<T> {
    /// Creates a new Clock Management driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Clock<T>) -> Self {
        Self { periph }
    }

    /// Releases the Clock Management peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Clock<T> {
        self.periph
    }

    /// Selects the glitchless multiplexer input.
    ///
    /// This function will block until the clock is selected by the glitchless
    /// mux.
    pub fn select_glitchless(&self, src: u32)
    where
        T: periph::clock::ClocksCtrlSrc,
    {
        self.periph.clocks_ctrl.store_reg(|r, v| {
            r.src().write(v, src);
        });
        spin_while!(self.periph.clocks_selected.load_bits() & 1 << src == 0);
    }
}
