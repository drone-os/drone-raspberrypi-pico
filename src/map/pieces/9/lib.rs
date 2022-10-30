#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports, rustdoc::broken_intra_doc_links)]
#![no_std]

#[doc(hidden)]
pub mod reg {
    #[allow(unused_imports)]
    use drone_core::reg;
    #[allow(unused_imports)]
    use drone_cortexm::reg::prelude::*;
    #[allow(unused_imports)]
    use drone_raspberrypi_pico_map_pieces_traits::*;

    include!(concat!(env!("OUT_DIR"), "/svd_regs.rs"));
}
