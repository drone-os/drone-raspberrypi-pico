//! Phase Locked Loops.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic PLL peripheral variant.
    pub trait PllMap {}

    /// Generic PLL peripheral.
    pub struct Pll;

    PLL {
        CS {
            0x20 RwRegAtomicAlias;
            LOCK { RoRwRegFieldBit }
            BYPASS { RwRwRegFieldBit }
            REFDIV { RwRwRegFieldBits }
        }
        PWR {
            0x20 RwRegAtomicAlias;
            VCOPD { RwRwRegFieldBit }
            POSTDIVPD { RwRwRegFieldBit }
            DSMPD { RwRwRegFieldBit }
            PD { RwRwRegFieldBit }
        }
        FBDIV_INT {
            0x20 RwRegAtomicAlias;
            FBDIV_INT { RwRwRegFieldBits }
        }
        PRIM {
            0x20 RwRegAtomicAlias;
            POSTDIV1 { RwRwRegFieldBits }
            POSTDIV2 { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_pll {
    ($pll_macro_doc:expr, $pll_macro:ident, $pll_ty_doc:expr, $pll_ty:ident, $pll:ident,) => {
        periph::map! {
            #[doc = $pll_macro_doc]
            pub macro $pll_macro;

            #[doc = $pll_ty_doc]
            pub struct $pll_ty;

            impl PllMap for $pll_ty {}

            crate::map::reg;
            crate::map::periph::pll;

            PLL {
                $pll;
                CS {
                    CS;
                    LOCK { LOCK }
                    BYPASS { BYPASS }
                    REFDIV { REFDIV }
                }
                PWR {
                    PWR;
                    VCOPD { VCOPD }
                    POSTDIVPD { POSTDIVPD }
                    DSMPD { DSMPD }
                    PD { PD }
                }
                FBDIV_INT {
                    FBDIV_INT;
                    FBDIV_INT { FBDIV_INT }
                }
                PRIM {
                    PRIM;
                    POSTDIV1 { POSTDIV1 }
                    POSTDIV2 { POSTDIV2 }
                }
            }
        }
    };
}

map_pll! {
    "Extracts PLL_SYS register tokens.",
    periph_pll_sys,
    "pll_sys peripheral variant.",
    PllSys,
    PLL_SYS,
}

map_pll! {
    "Extracts PLL_USB register tokens.",
    periph_pll_usb,
    "pll_usb peripheral variant.",
    PllUsb,
    PLL_USB,
}
