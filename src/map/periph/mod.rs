//! RP2040 peripheral mappings.

pub mod fc0;
pub mod watchdog;
pub mod xosc;

/// Clocks.
#[cfg(feature = "periph-clock")]
pub mod clock {
    pub use drone_raspberrypi_pico_map_periph_clock::*;
}

/// General Purpose Input / Output (GPIO) pins.
#[cfg(feature = "periph-gpio")]
pub mod gpio {
    pub use drone_raspberrypi_pico_map_periph_gpio::*;
}

/// Phase-locked loops.
#[cfg(feature = "periph-pll")]
pub mod pll {
    pub use drone_raspberrypi_pico_map_periph_pll::*;
}

#[doc(no_inline)]
pub use drone_cortexm::map::periph::*;
