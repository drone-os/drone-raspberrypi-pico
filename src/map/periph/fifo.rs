//! Inter-processor FIFOs (Mailboxes).

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts FIFO register tokens.
    pub macro periph_fifo;

    /// FIFO peripheral.
    pub struct Fifo;

    crate::map::reg;
    crate::map::periph::fifo;

    SIO {
        FIFO_ST;
        FIFO_WR;
        FIFO_RD;
    }
}
