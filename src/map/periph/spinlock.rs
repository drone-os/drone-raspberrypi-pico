//! Hardware Spinlocks.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic Hardware Spinlock peripheral variant.
    pub trait SpinlockMap {}

    /// Generic Hardware Spinlock peripheral.
    pub struct Spinlock;

    SIO {
        SPINLOCK_ST {
            0x20 RoReg Shared;
            SPINLOCK_ST { RoRoRegFieldBit }
        }
        SPINLOCK {
            0x20 RwReg;
        }
    }
}

macro_rules! map_spinlock {
    (
        $spinlock_macro_doc:expr,
        $spinlock_macro:ident,
        $spinlock_ty_doc:expr,
        $spinlock_ty:ident,
        $spinlock:ident,
        $spinlock_st:ident,
    ) => {
        periph::map! {
            #[doc = $spinlock_macro_doc]
            pub macro $spinlock_macro;

            #[doc = $spinlock_ty_doc]
            pub struct $spinlock_ty;

            impl SpinlockMap for $spinlock_ty {}

            crate::map::reg;
            crate::map::periph::spinlock;

            SIO {
                SPINLOCK_ST {
                    SPINLOCK_ST Shared;
                    SPINLOCK_ST { $spinlock_st }
                }
                SPINLOCK {
                    $spinlock;
                }
            }
        }
    };
}

map_spinlock! {
    "Extracts SPINLOCK0 register tokens.",
    periph_spinlock0,
    "spinlock0 peripheral variant.",
    Spinlock0,
    SPINLOCK0,
    SPINLOCK0_ST,
}

map_spinlock! {
    "Extracts SPINLOCK1 register tokens.",
    periph_spinlock1,
    "spinlock1 peripheral variant.",
    Spinlock1,
    SPINLOCK1,
    SPINLOCK1_ST,
}

map_spinlock! {
    "Extracts SPINLOCK2 register tokens.",
    periph_spinlock2,
    "spinlock2 peripheral variant.",
    Spinlock2,
    SPINLOCK2,
    SPINLOCK2_ST,
}

map_spinlock! {
    "Extracts SPINLOCK3 register tokens.",
    periph_spinlock3,
    "spinlock3 peripheral variant.",
    Spinlock3,
    SPINLOCK3,
    SPINLOCK3_ST,
}

map_spinlock! {
    "Extracts SPINLOCK4 register tokens.",
    periph_spinlock4,
    "spinlock4 peripheral variant.",
    Spinlock4,
    SPINLOCK4,
    SPINLOCK4_ST,
}

map_spinlock! {
    "Extracts SPINLOCK5 register tokens.",
    periph_spinlock5,
    "spinlock5 peripheral variant.",
    Spinlock5,
    SPINLOCK5,
    SPINLOCK5_ST,
}

map_spinlock! {
    "Extracts SPINLOCK6 register tokens.",
    periph_spinlock6,
    "spinlock6 peripheral variant.",
    Spinlock6,
    SPINLOCK6,
    SPINLOCK6_ST,
}

map_spinlock! {
    "Extracts SPINLOCK7 register tokens.",
    periph_spinlock7,
    "spinlock7 peripheral variant.",
    Spinlock7,
    SPINLOCK7,
    SPINLOCK7_ST,
}

map_spinlock! {
    "Extracts SPINLOCK8 register tokens.",
    periph_spinlock8,
    "spinlock8 peripheral variant.",
    Spinlock8,
    SPINLOCK8,
    SPINLOCK8_ST,
}

map_spinlock! {
    "Extracts SPINLOCK9 register tokens.",
    periph_spinlock9,
    "spinlock9 peripheral variant.",
    Spinlock9,
    SPINLOCK9,
    SPINLOCK9_ST,
}

map_spinlock! {
    "Extracts SPINLOCK10 register tokens.",
    periph_spinlock10,
    "spinlock10 peripheral variant.",
    Spinlock10,
    SPINLOCK10,
    SPINLOCK10_ST,
}

map_spinlock! {
    "Extracts SPINLOCK11 register tokens.",
    periph_spinlock11,
    "spinlock11 peripheral variant.",
    Spinlock11,
    SPINLOCK11,
    SPINLOCK11_ST,
}

map_spinlock! {
    "Extracts SPINLOCK12 register tokens.",
    periph_spinlock12,
    "spinlock12 peripheral variant.",
    Spinlock12,
    SPINLOCK12,
    SPINLOCK12_ST,
}

map_spinlock! {
    "Extracts SPINLOCK13 register tokens.",
    periph_spinlock13,
    "spinlock13 peripheral variant.",
    Spinlock13,
    SPINLOCK13,
    SPINLOCK13_ST,
}

map_spinlock! {
    "Extracts SPINLOCK14 register tokens.",
    periph_spinlock14,
    "spinlock14 peripheral variant.",
    Spinlock14,
    SPINLOCK14,
    SPINLOCK14_ST,
}

map_spinlock! {
    "Extracts SPINLOCK15 register tokens.",
    periph_spinlock15,
    "spinlock15 peripheral variant.",
    Spinlock15,
    SPINLOCK15,
    SPINLOCK15_ST,
}

map_spinlock! {
    "Extracts SPINLOCK16 register tokens.",
    periph_spinlock16,
    "spinlock16 peripheral variant.",
    Spinlock16,
    SPINLOCK16,
    SPINLOCK16_ST,
}

map_spinlock! {
    "Extracts SPINLOCK17 register tokens.",
    periph_spinlock17,
    "spinlock17 peripheral variant.",
    Spinlock17,
    SPINLOCK17,
    SPINLOCK17_ST,
}

map_spinlock! {
    "Extracts SPINLOCK18 register tokens.",
    periph_spinlock18,
    "spinlock18 peripheral variant.",
    Spinlock18,
    SPINLOCK18,
    SPINLOCK18_ST,
}

map_spinlock! {
    "Extracts SPINLOCK19 register tokens.",
    periph_spinlock19,
    "spinlock19 peripheral variant.",
    Spinlock19,
    SPINLOCK19,
    SPINLOCK19_ST,
}

map_spinlock! {
    "Extracts SPINLOCK20 register tokens.",
    periph_spinlock20,
    "spinlock20 peripheral variant.",
    Spinlock20,
    SPINLOCK20,
    SPINLOCK20_ST,
}

map_spinlock! {
    "Extracts SPINLOCK21 register tokens.",
    periph_spinlock21,
    "spinlock21 peripheral variant.",
    Spinlock21,
    SPINLOCK21,
    SPINLOCK21_ST,
}

map_spinlock! {
    "Extracts SPINLOCK22 register tokens.",
    periph_spinlock22,
    "spinlock22 peripheral variant.",
    Spinlock22,
    SPINLOCK22,
    SPINLOCK22_ST,
}

map_spinlock! {
    "Extracts SPINLOCK23 register tokens.",
    periph_spinlock23,
    "spinlock23 peripheral variant.",
    Spinlock23,
    SPINLOCK23,
    SPINLOCK23_ST,
}

map_spinlock! {
    "Extracts SPINLOCK24 register tokens.",
    periph_spinlock24,
    "spinlock24 peripheral variant.",
    Spinlock24,
    SPINLOCK24,
    SPINLOCK24_ST,
}

map_spinlock! {
    "Extracts SPINLOCK25 register tokens.",
    periph_spinlock25,
    "spinlock25 peripheral variant.",
    Spinlock25,
    SPINLOCK25,
    SPINLOCK25_ST,
}

map_spinlock! {
    "Extracts SPINLOCK26 register tokens.",
    periph_spinlock26,
    "spinlock26 peripheral variant.",
    Spinlock26,
    SPINLOCK26,
    SPINLOCK26_ST,
}

map_spinlock! {
    "Extracts SPINLOCK27 register tokens.",
    periph_spinlock27,
    "spinlock27 peripheral variant.",
    Spinlock27,
    SPINLOCK27,
    SPINLOCK27_ST,
}

map_spinlock! {
    "Extracts SPINLOCK28 register tokens.",
    periph_spinlock28,
    "spinlock28 peripheral variant.",
    Spinlock28,
    SPINLOCK28,
    SPINLOCK28_ST,
}

map_spinlock! {
    "Extracts SPINLOCK29 register tokens.",
    periph_spinlock29,
    "spinlock29 peripheral variant.",
    Spinlock29,
    SPINLOCK29,
    SPINLOCK29_ST,
}

map_spinlock! {
    "Extracts SPINLOCK30 register tokens.",
    periph_spinlock30,
    "spinlock30 peripheral variant.",
    Spinlock30,
    SPINLOCK30,
    SPINLOCK30_ST,
}

map_spinlock! {
    "Extracts SPINLOCK31 register tokens.",
    periph_spinlock31,
    "spinlock31 peripheral variant.",
    Spinlock31,
    SPINLOCK31,
    SPINLOCK31_ST,
}
