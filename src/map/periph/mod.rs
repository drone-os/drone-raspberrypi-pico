//! RP2040 peripheral mappings.

pub mod clock;
pub mod div;
pub mod fc0;
pub mod fifo;
pub mod gpio;
pub mod interp;
pub mod pll;
pub mod resets;
pub mod spinlock;
pub mod watchdog;
pub mod xosc;

pub use self::clock::{Clock, ClockMap};
pub use self::div::Div;
pub use self::fc0::Fc0;
pub use self::fifo::Fifo;
pub use self::gpio::{Gpio, GpioMap};
pub use self::interp::{Interp, InterpMap};
pub use self::pll::{Pll, PllMap};
pub use self::resets::Resets;
pub use self::spinlock::{Spinlock, SpinlockMap};
pub use self::watchdog::Watchdog;
pub use self::xosc::Xosc;
