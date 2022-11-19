//! Direct Memory Access (DMA) controller.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts DMA controller register tokens.
    pub macro periph_dma;

    /// DMA controller peripheral.
    pub struct Dma;

    crate::map::reg;
    crate::map::periph::dma;

    DMA {
        TIMER0;
        TIMER1;
        TIMER2;
        TIMER3;
        SNIFF_CTRL;
        SNIFF_DATA;
        FIFO_LEVELS;
        N_CHANNELS;
    }
}
