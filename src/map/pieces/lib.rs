//! RP2040 memory-mapped register bindings for Drone, an Embedded Operating
//! System.

#![feature(marker_trait_attr)]
#![feature(proc_macro_hygiene)]
#![warn(clippy::pedantic)]
#![allow(broken_intra_doc_links)]
#![no_std]

#[doc(hidden)]
pub mod reg {
    mod inner {
        pub use drone_raspberrypi_pico_map_pieces_1::reg::*;
    }

    #[allow(unused_imports)]
    use drone_core::reg;

    include!(concat!(env!("OUT_DIR"), "/svd_reg_index.rs"));
}

#[doc(hidden)]
pub mod thr {
    mod map {
        #[allow(unused_imports)]
        use drone_cortexm::thr;
        #[allow(unused_imports)]
        use drone_cortexm::thr::prelude::*;
    }

    pub use self::map::*;
}
