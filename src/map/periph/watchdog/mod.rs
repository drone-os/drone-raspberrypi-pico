//! Watchdog.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Watchdog register tokens.
    pub macro periph_watchdog;

    /// Watchdog peripheral.
    pub struct Watchdog;

    crate::map::reg;
    crate::map::periph::watchdog;

    WATCHDOG {
        CTRL;
        LOAD;
        REASON;
        SCRATCH0;
        SCRATCH1;
        SCRATCH2;
        SCRATCH3;
        SCRATCH4;
        SCRATCH5;
        SCRATCH6;
        SCRATCH7;
        TICK;
    }
}
