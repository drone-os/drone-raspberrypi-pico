//! Interpolators.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic Interpolator peripheral variant.
    pub trait InterpMap {}

    /// Generic Interpolator peripheral.
    pub struct Interp;

    SIO {
        INTERP_ACCUM0 {
            0x20 RwReg;
        }
        INTERP_ACCUM1 {
            0x20 RwReg;
        }
        INTERP_BASE0 {
            0x20 RwReg;
        }
        INTERP_BASE1 {
            0x20 RwReg;
        }
        INTERP_BASE2 {
            0x20 RwReg;
        }
        INTERP_POP_LANE0 {
            0x20 RoReg;
        }
        INTERP_POP_LANE1 {
            0x20 RoReg;
        }
        INTERP_POP_FULL {
            0x20 RoReg;
        }
        INTERP_PEEK_LANE0 {
            0x20 RoReg;
        }
        INTERP_PEEK_LANE1 {
            0x20 RoReg;
        }
        INTERP_PEEK_FULL {
            0x20 RoReg;
        }
        INTERP_CTRL_LANE0 {
            0x20 RwReg;
            OVERF { RoRwRegFieldBit }
            OVERF1 { RoRwRegFieldBit }
            OVERF0 { RoRwRegFieldBit }
            BLEND { RwRwRegFieldBit Option }
            CLAMP { RwRwRegFieldBit Option }
            FORCE_MSB { RwRwRegFieldBits }
            ADD_RAW { RwRwRegFieldBit }
            CROSS_RESULT { RwRwRegFieldBit }
            CROSS_INPUT { RwRwRegFieldBit }
            SIGNED { RwRwRegFieldBit }
            MASK_MSB { RwRwRegFieldBits }
            MASK_LSB { RwRwRegFieldBits }
            SHIFT { RwRwRegFieldBits }
        }
        INTERP_CTRL_LANE1 {
            0x20 RwReg;
            FORCE_MSB { RwRwRegFieldBits }
            ADD_RAW { RwRwRegFieldBit }
            CROSS_RESULT { RwRwRegFieldBit }
            CROSS_INPUT { RwRwRegFieldBit }
            SIGNED { RwRwRegFieldBit }
            MASK_MSB { RwRwRegFieldBits }
            MASK_LSB { RwRwRegFieldBits }
            SHIFT { RwRwRegFieldBits }
        }
        INTERP_ACCUM0_ADD {
            0x20 RwReg;
            INTERP_ACCUM0_ADD { RwRwRegFieldBits }
        }
        INTERP_ACCUM1_ADD {
            0x20 RwReg;
            INTERP_ACCUM1_ADD { RwRwRegFieldBits }
        }
        INTERP_BASE_1AND0 {
            0x20 WoReg;
        }
    }
}

macro_rules! map_interp {
    (
        $interp_macro_doc:expr,
        $interp_macro:ident,
        $interp_ty_doc:expr,
        $interp_ty:ident,
        $interp_accum0:ident,
        $interp_accum1:ident,
        $interp_base0:ident,
        $interp_base1:ident,
        $interp_base2:ident,
        $interp_pop_lane0:ident,
        $interp_pop_lane1:ident,
        $interp_pop_full:ident,
        $interp_peek_lane0:ident,
        $interp_peek_lane1:ident,
        $interp_peek_full:ident,
        (
            $interp_ctrl_lane0:ident,
            $($blend:ident)?,
            $($clamp:ident)?
        ),
        $interp_ctrl_lane1:ident,
        $interp_accum0_add:ident,
        $interp_accum1_add:ident,
        $interp_base_1and0:ident,
    ) => {
        periph::map! {
            #[doc = $interp_macro_doc]
            pub macro $interp_macro;

            #[doc = $interp_ty_doc]
            pub struct $interp_ty;

            impl InterpMap for $interp_ty {}

            crate::map::reg;
            crate::map::periph::interp;

            SIO {
                INTERP_ACCUM0 {
                    $interp_accum0;
                }
                INTERP_ACCUM1 {
                    $interp_accum1;
                }
                INTERP_BASE0 {
                    $interp_base0;
                }
                INTERP_BASE1 {
                    $interp_base1;
                }
                INTERP_BASE2 {
                    $interp_base2;
                }
                INTERP_POP_LANE0 {
                    $interp_pop_lane0;
                }
                INTERP_POP_LANE1 {
                    $interp_pop_lane1;
                }
                INTERP_POP_FULL {
                    $interp_pop_full;
                }
                INTERP_PEEK_LANE0 {
                    $interp_peek_lane0;
                }
                INTERP_PEEK_LANE1 {
                    $interp_peek_lane1;
                }
                INTERP_PEEK_FULL {
                    $interp_peek_full;
                }
                INTERP_CTRL_LANE0 {
                    $interp_ctrl_lane0;
                    OVERF { OVERF }
                    OVERF1 { OVERF1 }
                    OVERF0 { OVERF0 }
                    BLEND { $($blend Option)* }
                    CLAMP { $($clamp Option)* }
                    FORCE_MSB { FORCE_MSB }
                    ADD_RAW { ADD_RAW }
                    CROSS_RESULT { CROSS_RESULT }
                    CROSS_INPUT { CROSS_INPUT }
                    SIGNED { SIGNED }
                    MASK_MSB { MASK_MSB }
                    MASK_LSB { MASK_LSB }
                    SHIFT { SHIFT }
                }
                INTERP_CTRL_LANE1 {
                    $interp_ctrl_lane1;
                    FORCE_MSB { FORCE_MSB }
                    ADD_RAW { ADD_RAW }
                    CROSS_RESULT { CROSS_RESULT }
                    CROSS_INPUT { CROSS_INPUT }
                    SIGNED { SIGNED }
                    MASK_MSB { MASK_MSB }
                    MASK_LSB { MASK_LSB }
                    SHIFT { SHIFT }
                }
                INTERP_ACCUM0_ADD {
                    $interp_accum0_add;
                    INTERP_ACCUM0_ADD { $interp_accum0_add }
                }
                INTERP_ACCUM1_ADD {
                    $interp_accum1_add;
                    INTERP_ACCUM1_ADD { $interp_accum1_add }
                }
                INTERP_BASE_1AND0 {
                    $interp_base_1and0;
                }
            }
        }
    };
}

map_interp! {
    "Extracts INTERP0 register tokens.",
    periph_interp0,
    "interp0 peripheral variant.",
    Interp0,
    INTERP0_ACCUM0,
    INTERP0_ACCUM1,
    INTERP0_BASE0,
    INTERP0_BASE1,
    INTERP0_BASE2,
    INTERP0_POP_LANE0,
    INTERP0_POP_LANE1,
    INTERP0_POP_FULL,
    INTERP0_PEEK_LANE0,
    INTERP0_PEEK_LANE1,
    INTERP0_PEEK_FULL,
    (INTERP0_CTRL_LANE0, BLEND,),
    INTERP0_CTRL_LANE1,
    INTERP0_ACCUM0_ADD,
    INTERP0_ACCUM1_ADD,
    INTERP0_BASE_1AND0,
}

map_interp! {
    "Extracts INTERP1 register tokens.",
    periph_interp1,
    "interp1 peripheral variant.",
    Interp1,
    INTERP1_ACCUM0,
    INTERP1_ACCUM1,
    INTERP1_BASE0,
    INTERP1_BASE1,
    INTERP1_BASE2,
    INTERP1_POP_LANE0,
    INTERP1_POP_LANE1,
    INTERP1_POP_FULL,
    INTERP1_PEEK_LANE0,
    INTERP1_PEEK_LANE1,
    INTERP1_PEEK_FULL,
    (INTERP1_CTRL_LANE0,, CLAMP),
    INTERP1_CTRL_LANE1,
    INTERP1_ACCUM0_ADD,
    INTERP1_ACCUM1_ADD,
    INTERP1_BASE_1AND0,
}
