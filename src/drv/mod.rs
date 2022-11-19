//! Drivers for core Raspberry Pi Pico peripherals.

pub mod clock;
pub mod freq_counter;
pub mod pll;
pub mod resets;
pub mod watchdog;
pub mod xosc;

pub use self::clock::Clock;
pub use self::freq_counter::FreqCounter;
pub use self::pll::Pll;
pub use self::resets::Resets;
pub use self::watchdog::Watchdog;
pub use self::xosc::Xosc;
