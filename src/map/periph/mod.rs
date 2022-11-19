//! RP2040 peripheral mappings.

pub mod clock;
pub mod fc0;
pub mod gpio;
pub mod pll;
pub mod resets;
pub mod watchdog;
pub mod xosc;

pub use self::clock::{Clock, ClockMap};
pub use self::fc0::Fc0;
pub use self::gpio::{Gpio, GpioMap};
pub use self::pll::{Pll, PllMap};
pub use self::resets::Resets;
pub use self::watchdog::Watchdog;
pub use self::xosc::Xosc;
