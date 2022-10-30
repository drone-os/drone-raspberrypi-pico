//! RP2040 peripheral mappings.

#[doc(no_inline)]
pub use drone_cortexm::map::periph::*;

/// General Purpose I/Os.
#[cfg(feature = "gpio")]
pub mod gpio {
    pub use drone_raspberrypi_pico_map_periph_gpio::*;
}
