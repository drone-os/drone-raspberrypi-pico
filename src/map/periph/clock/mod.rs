//! Clocks.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic Clock peripheral variant.
    pub trait ClockMap {}

    /// Generic Clock peripheral.
    pub struct Clock;

    CLOCKS {
        CTRL {
            0x20 RwRegAtomicAlias;
            NUDGE { RwRwRegFieldBit Option }
            PHASE { RwRwRegFieldBits Option }
            DC50 { RwRwRegFieldBit Option }
            ENABLE { RwRwRegFieldBit Option }
            KILL { RwRwRegFieldBit Option }
            AUXSRC { RwRwRegFieldBits }
            SRC { RwRwRegFieldBits Option }
        }
        DIV {
            0x20 RwRegAtomicAlias Option;
            INT { RwRwRegFieldBits }
            FRAC { RwRwRegFieldBits Option }
        }
        SELECTED {
            0x20 RoRegAtomicAlias;
        }
        RESUS_CTRL {
            0x20 RwRegAtomicAlias Option;
            CLEAR { RwRwRegFieldBit }
            FRCE { RwRwRegFieldBit }
            ENABLE { RwRwRegFieldBit }
            TIMEOUT { RwRwRegFieldBits }
        }
        RESUS_STATUS {
            0x20 RoRegAtomicAlias Option;
            RESUSSED { RoRoRegFieldBit }
        }
    }
}

macro_rules! map_clock {
    (
        $clock_macro_doc:expr,
        $clock_macro:ident,
        $clock_ty_doc:expr,
        $clock_ty:ident,
        (
            $clk_ctrl:ident,
            $($nudge:ident)?,
            $($phase:ident)?,
            $($dc50:ident)?,
            $($enable:ident)?,
            $($kill:ident)?,
            $($src:ident)?
        ),
        ($(
            $clk_div:ident,
            $($frac:ident)?
        )?),
        $clk_selected:ident,
        ($($resus_ctrl:ident)?),
        ($($resus_status:ident)?),
    ) => {
        periph::map! {
            #[doc = $clock_macro_doc]
            pub macro $clock_macro;

            #[doc = $clock_ty_doc]
            pub struct $clock_ty;

            impl ClockMap for $clock_ty {}

            crate::map::reg;
            crate::map::periph::clock;

            CLOCKS {
                CTRL {
                    $clk_ctrl;
                    NUDGE { $($nudge Option)* }
                    PHASE { $($phase Option)* }
                    DC50 { $($dc50 Option)* }
                    ENABLE { $($enable Option)* }
                    KILL { $($kill Option)* }
                    AUXSRC { AUXSRC }
                    SRC { $($src Option)* }
                }
                DIV {
                    $(
                        $clk_div Option;
                        INT { INT }
                        FRAC { $($frac Option)* }
                    )*
                }
                SELECTED {
                    $clk_selected;
                }
                RESUS_CTRL {
                    $(
                        $resus_ctrl Option;
                        CLEAR { CLEAR }
                        FRCE { FRCE }
                        ENABLE { ENABLE }
                        TIMEOUT { TIMEOUT }
                    )*
               }
                RESUS_STATUS {
                    $(
                        $resus_status Option;
                        RESUSSED { RESUSSED }
                    )*
               }
            }
        }
    };
}

map_clock! {
    "Extracts CLK_GPOUT0 register tokens.",
    periph_clk_gpout0,
    "clk_gpout0 peripheral variant.",
    ClkGpout0,
    (CLK_GPOUT0_CTRL, NUDGE, PHASE, DC50, ENABLE, KILL,),
    (CLK_GPOUT0_DIV, FRAC),
    CLK_GPOUT0_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_GPOUT1 register tokens.",
    periph_clk_gpout1,
    "clk_gpout1 peripheral variant.",
    ClkGpout1,
    (CLK_GPOUT1_CTRL, NUDGE, PHASE, DC50, ENABLE, KILL,),
    (CLK_GPOUT1_DIV, FRAC),
    CLK_GPOUT1_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_GPOUT2 register tokens.",
    periph_clk_gpout2,
    "clk_gpout2 peripheral variant.",
    ClkGpout2,
    (CLK_GPOUT2_CTRL, NUDGE, PHASE, DC50, ENABLE, KILL,),
    (CLK_GPOUT2_DIV, FRAC),
    CLK_GPOUT2_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_GPOUT3 register tokens.",
    periph_clk_gpout3,
    "clk_gpout3 peripheral variant.",
    ClkGpout3,
    (CLK_GPOUT3_CTRL, NUDGE, PHASE, DC50, ENABLE, KILL,),
    (CLK_GPOUT3_DIV, FRAC),
    CLK_GPOUT3_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_REF register tokens.",
    periph_clk_ref,
    "clk_ref peripheral variant.",
    ClkRef,
    (CLK_REF_CTRL,,,,,,SRC),
    (CLK_REF_DIV,),
    CLK_REF_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_SYS register tokens.",
    periph_clk_sys,
    "clk_sys peripheral variant.",
    ClkSys,
    (CLK_SYS_CTRL,,,,,,SRC),
    (CLK_SYS_DIV,FRAC),
    CLK_SYS_SELECTED,
    (CLK_SYS_RESUS_CTRL),
    (CLK_SYS_RESUS_STATUS),
}

map_clock! {
    "Extracts CLK_PERI register tokens.",
    periph_clk_peri,
    "clk_peri peripheral variant.",
    ClkPeri,
    (CLK_PERI_CTRL,,,,ENABLE,KILL,),
    (),
    CLK_PERI_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_USB register tokens.",
    periph_clk_usb,
    "clk_usb peripheral variant.",
    ClkUsb,
    (CLK_USB_CTRL,NUDGE,PHASE,,ENABLE,KILL,),
    (CLK_USB_DIV,),
    CLK_USB_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_ADC register tokens.",
    periph_clk_adc,
    "clk_adc peripheral variant.",
    ClkAdc,
    (CLK_ADC_CTRL,NUDGE,PHASE,,ENABLE,KILL,),
    (CLK_ADC_DIV,),
    CLK_ADC_SELECTED,
    (),
    (),
}

map_clock! {
    "Extracts CLK_RTC register tokens.",
    periph_clk_rtc,
    "clk_rtc peripheral variant.",
    ClkRtc,
    (CLK_RTC_CTRL,NUDGE,PHASE,,ENABLE,KILL,),
    (CLK_RTC_DIV,FRAC),
    CLK_RTC_SELECTED,
    (),
    (),
}
