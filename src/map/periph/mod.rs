//! RP2040 peripheral mappings.

pub mod clock;
pub mod div;
pub mod dma;
pub mod dma_ch;
pub mod fc0;
pub mod fifo;
pub mod gpio;
pub mod interp;
pub mod pads_bank0;
pub mod pads_qspi;
pub mod pll;
pub mod resets;
pub mod rosc;
pub mod spi;
pub mod spinlock;
pub mod watchdog;
pub mod xosc;

pub use self::clock::{Clock, ClockMap};
pub use self::div::Div;
pub use self::dma::Dma;
pub use self::dma_ch::{DmaCh, DmaChMap};
pub use self::fc0::Fc0;
pub use self::fifo::Fifo;
pub use self::gpio::{Gpio, GpioMap};
pub use self::interp::{Interp, InterpMap};
pub use self::pads_bank0::PadsBank0;
pub use self::pads_qspi::PadsQspi;
pub use self::pll::{Pll, PllMap};
pub use self::resets::Resets;
pub use self::rosc::Rosc;
pub use self::spi::{Spi, SpiMap};
pub use self::spinlock::{Spinlock, SpinlockMap};
pub use self::watchdog::Watchdog;
pub use self::xosc::Xosc;
