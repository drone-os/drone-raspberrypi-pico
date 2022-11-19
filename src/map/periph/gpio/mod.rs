//! General Purpose Input / Output (GPIO) pins.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic GPIO peripheral variant.
    pub trait GpioMap {}

    /// Generic GPIO peripheral.
    pub struct Gpio;

    IO_BANK0 {
        GPIO_CTRL {
            0x20 RwRegAtomicAlias;
            IRQOVER { RwRwRegFieldBits }
            INOVER { RwRwRegFieldBits }
            OEOVER { RwRwRegFieldBits }
            OUTOVER { RwRwRegFieldBits }
            FUNCSEL { RwRwRegFieldBits }
        }
    }

    SIO {
        GPIO_IN {
            0x20 RoReg Shared;
            GPIO_IN { RoRoRegFieldBit }
        }
        GPIO_OUT {
            0x20 RwReg Shared;
            GPIO_OUT { RwRwRegFieldBit }
        }
        GPIO_OUT_SET {
            0x20 WoReg Shared;
            GPIO_OUT_SET { WoWoRegFieldBit }
        }
        GPIO_OUT_CLR {
            0x20 WoReg Shared;
            GPIO_OUT_CLR { WoWoRegFieldBit }
        }
        GPIO_OUT_XOR {
            0x20 WoReg Shared;
            GPIO_OUT_XOR { WoWoRegFieldBit }
        }
        GPIO_OE {
            0x20 RwReg Shared;
            GPIO_OE { RwRwRegFieldBit }
        }
        GPIO_OE_SET {
            0x20 WoReg Shared;
            GPIO_OE_SET { WoWoRegFieldBit }
        }
        GPIO_OE_CLR {
            0x20 WoReg Shared;
            GPIO_OE_CLR { WoWoRegFieldBit }
        }
        GPIO_OE_XOR {
            0x20 WoReg Shared;
            GPIO_OE_XOR { WoWoRegFieldBit }
        }
    }
}

macro_rules! map_gpio {
    (
        $gpio_macro_doc:expr,
        $gpio_macro:ident,
        $gpio_ty_doc:expr,
        $gpio_ty:ident,
        $gpio_ctrl:ident,
        $gpio_in_ty:ident,
        $gpio_out_ty:ident,
        $gpio_out_set_ty:ident,
        $gpio_out_clr_ty:ident,
        $gpio_out_xor_ty:ident,
        $gpio_oe_ty:ident,
        $gpio_oe_set_ty:ident,
        $gpio_oe_clr_ty:ident,
        $gpio_oe_xor_ty:ident,
    ) => {
        periph::map! {
            #[doc = $gpio_macro_doc]
            pub macro $gpio_macro;

            #[doc = $gpio_ty_doc]
            pub struct $gpio_ty;

            impl GpioMap for $gpio_ty {}

            crate::map::reg;
            crate::map::periph::gpio;

            IO_BANK0 {
                GPIO_CTRL {
                    $gpio_ctrl;
                    IRQOVER { IRQOVER }
                    INOVER { INOVER }
                    OEOVER { OEOVER }
                    OUTOVER { OUTOVER }
                    FUNCSEL { FUNCSEL }
                }
            }

            SIO {
                GPIO_IN {
                    GPIO_IN Shared;
                    GPIO_IN { $gpio_in_ty }
                }
                GPIO_OUT {
                    GPIO_OUT Shared;
                    GPIO_OUT { $gpio_out_ty }
                }
                GPIO_OUT_SET {
                    GPIO_OUT_SET Shared;
                    GPIO_OUT_SET { $gpio_out_set_ty }
                }
                GPIO_OUT_CLR {
                    GPIO_OUT_CLR Shared;
                    GPIO_OUT_CLR { $gpio_out_clr_ty }
                }
                GPIO_OUT_XOR {
                    GPIO_OUT_XOR Shared;
                    GPIO_OUT_XOR { $gpio_out_xor_ty }
                }
                GPIO_OE {
                    GPIO_OE Shared;
                    GPIO_OE { $gpio_oe_ty }
                }
                GPIO_OE_SET {
                    GPIO_OE_SET Shared;
                    GPIO_OE_SET { $gpio_oe_set_ty }
                }
                GPIO_OE_CLR {
                    GPIO_OE_CLR Shared;
                    GPIO_OE_CLR { $gpio_oe_clr_ty }
                }
                GPIO_OE_XOR {
                    GPIO_OE_XOR Shared;
                    GPIO_OE_XOR { $gpio_oe_xor_ty }
                }
            }
        }
    };
}

map_gpio! {
    "Extracts GPIO0 register tokens.",
    periph_gpio0,
    "GPIO0 peripheral variant.",
    Gpio0,
    GPIO0_CTRL,
    GPIO0_IN,
    GPIO0_OUT,
    GPIO0_OUT_SET,
    GPIO0_OUT_CLR,
    GPIO0_OUT_XOR,
    GPIO0_OE,
    GPIO0_OE_SET,
    GPIO0_OE_CLR,
    GPIO0_OE_XOR,
}

map_gpio! {
    "Extracts GPIO1 register tokens.",
    periph_gpio1,
    "GPIO1 peripheral variant.",
    Gpio1,
    GPIO1_CTRL,
    GPIO1_IN,
    GPIO1_OUT,
    GPIO1_OUT_SET,
    GPIO1_OUT_CLR,
    GPIO1_OUT_XOR,
    GPIO1_OE,
    GPIO1_OE_SET,
    GPIO1_OE_CLR,
    GPIO1_OE_XOR,
}

map_gpio! {
    "Extracts GPIO2 register tokens.",
    periph_gpio2,
    "GPIO2 peripheral variant.",
    Gpio2,
    GPIO2_CTRL,
    GPIO2_IN,
    GPIO2_OUT,
    GPIO2_OUT_SET,
    GPIO2_OUT_CLR,
    GPIO2_OUT_XOR,
    GPIO2_OE,
    GPIO2_OE_SET,
    GPIO2_OE_CLR,
    GPIO2_OE_XOR,
}

map_gpio! {
    "Extracts GPIO3 register tokens.",
    periph_gpio3,
    "GPIO3 peripheral variant.",
    Gpio3,
    GPIO3_CTRL,
    GPIO3_IN,
    GPIO3_OUT,
    GPIO3_OUT_SET,
    GPIO3_OUT_CLR,
    GPIO3_OUT_XOR,
    GPIO3_OE,
    GPIO3_OE_SET,
    GPIO3_OE_CLR,
    GPIO3_OE_XOR,
}

map_gpio! {
    "Extracts GPIO4 register tokens.",
    periph_gpio4,
    "GPIO4 peripheral variant.",
    Gpio4,
    GPIO4_CTRL,
    GPIO4_IN,
    GPIO4_OUT,
    GPIO4_OUT_SET,
    GPIO4_OUT_CLR,
    GPIO4_OUT_XOR,
    GPIO4_OE,
    GPIO4_OE_SET,
    GPIO4_OE_CLR,
    GPIO4_OE_XOR,
}

map_gpio! {
    "Extracts GPIO5 register tokens.",
    periph_gpio5,
    "GPIO5 peripheral variant.",
    Gpio5,
    GPIO5_CTRL,
    GPIO5_IN,
    GPIO5_OUT,
    GPIO5_OUT_SET,
    GPIO5_OUT_CLR,
    GPIO5_OUT_XOR,
    GPIO5_OE,
    GPIO5_OE_SET,
    GPIO5_OE_CLR,
    GPIO5_OE_XOR,
}

map_gpio! {
    "Extracts GPIO6 register tokens.",
    periph_gpio6,
    "GPIO6 peripheral variant.",
    Gpio6,
    GPIO6_CTRL,
    GPIO6_IN,
    GPIO6_OUT,
    GPIO6_OUT_SET,
    GPIO6_OUT_CLR,
    GPIO6_OUT_XOR,
    GPIO6_OE,
    GPIO6_OE_SET,
    GPIO6_OE_CLR,
    GPIO6_OE_XOR,
}

map_gpio! {
    "Extracts GPIO7 register tokens.",
    periph_gpio7,
    "GPIO7 peripheral variant.",
    Gpio7,
    GPIO7_CTRL,
    GPIO7_IN,
    GPIO7_OUT,
    GPIO7_OUT_SET,
    GPIO7_OUT_CLR,
    GPIO7_OUT_XOR,
    GPIO7_OE,
    GPIO7_OE_SET,
    GPIO7_OE_CLR,
    GPIO7_OE_XOR,
}

map_gpio! {
    "Extracts GPIO8 register tokens.",
    periph_gpio8,
    "GPIO8 peripheral variant.",
    Gpio8,
    GPIO8_CTRL,
    GPIO8_IN,
    GPIO8_OUT,
    GPIO8_OUT_SET,
    GPIO8_OUT_CLR,
    GPIO8_OUT_XOR,
    GPIO8_OE,
    GPIO8_OE_SET,
    GPIO8_OE_CLR,
    GPIO8_OE_XOR,
}

map_gpio! {
    "Extracts GPIO9 register tokens.",
    periph_gpio9,
    "GPIO9 peripheral variant.",
    Gpio9,
    GPIO9_CTRL,
    GPIO9_IN,
    GPIO9_OUT,
    GPIO9_OUT_SET,
    GPIO9_OUT_CLR,
    GPIO9_OUT_XOR,
    GPIO9_OE,
    GPIO9_OE_SET,
    GPIO9_OE_CLR,
    GPIO9_OE_XOR,
}

map_gpio! {
    "Extracts GPIO10 register tokens.",
    periph_gpio10,
    "GPIO10 peripheral variant.",
    Gpio10,
    GPIO10_CTRL,
    GPIO10_IN,
    GPIO10_OUT,
    GPIO10_OUT_SET,
    GPIO10_OUT_CLR,
    GPIO10_OUT_XOR,
    GPIO10_OE,
    GPIO10_OE_SET,
    GPIO10_OE_CLR,
    GPIO10_OE_XOR,
}

map_gpio! {
    "Extracts GPIO11 register tokens.",
    periph_gpio11,
    "GPIO11 peripheral variant.",
    Gpio11,
    GPIO11_CTRL,
    GPIO11_IN,
    GPIO11_OUT,
    GPIO11_OUT_SET,
    GPIO11_OUT_CLR,
    GPIO11_OUT_XOR,
    GPIO11_OE,
    GPIO11_OE_SET,
    GPIO11_OE_CLR,
    GPIO11_OE_XOR,
}

map_gpio! {
    "Extracts GPIO12 register tokens.",
    periph_gpio12,
    "GPIO12 peripheral variant.",
    Gpio12,
    GPIO12_CTRL,
    GPIO12_IN,
    GPIO12_OUT,
    GPIO12_OUT_SET,
    GPIO12_OUT_CLR,
    GPIO12_OUT_XOR,
    GPIO12_OE,
    GPIO12_OE_SET,
    GPIO12_OE_CLR,
    GPIO12_OE_XOR,
}

map_gpio! {
    "Extracts GPIO13 register tokens.",
    periph_gpio13,
    "GPIO13 peripheral variant.",
    Gpio13,
    GPIO13_CTRL,
    GPIO13_IN,
    GPIO13_OUT,
    GPIO13_OUT_SET,
    GPIO13_OUT_CLR,
    GPIO13_OUT_XOR,
    GPIO13_OE,
    GPIO13_OE_SET,
    GPIO13_OE_CLR,
    GPIO13_OE_XOR,
}

map_gpio! {
    "Extracts GPIO14 register tokens.",
    periph_gpio14,
    "GPIO14 peripheral variant.",
    Gpio14,
    GPIO14_CTRL,
    GPIO14_IN,
    GPIO14_OUT,
    GPIO14_OUT_SET,
    GPIO14_OUT_CLR,
    GPIO14_OUT_XOR,
    GPIO14_OE,
    GPIO14_OE_SET,
    GPIO14_OE_CLR,
    GPIO14_OE_XOR,
}

map_gpio! {
    "Extracts GPIO15 register tokens.",
    periph_gpio15,
    "GPIO15 peripheral variant.",
    Gpio15,
    GPIO15_CTRL,
    GPIO15_IN,
    GPIO15_OUT,
    GPIO15_OUT_SET,
    GPIO15_OUT_CLR,
    GPIO15_OUT_XOR,
    GPIO15_OE,
    GPIO15_OE_SET,
    GPIO15_OE_CLR,
    GPIO15_OE_XOR,
}

map_gpio! {
    "Extracts GPIO16 register tokens.",
    periph_gpio16,
    "GPIO16 peripheral variant.",
    Gpio16,
    GPIO16_CTRL,
    GPIO16_IN,
    GPIO16_OUT,
    GPIO16_OUT_SET,
    GPIO16_OUT_CLR,
    GPIO16_OUT_XOR,
    GPIO16_OE,
    GPIO16_OE_SET,
    GPIO16_OE_CLR,
    GPIO16_OE_XOR,
}

map_gpio! {
    "Extracts GPIO17 register tokens.",
    periph_gpio17,
    "GPIO17 peripheral variant.",
    Gpio17,
    GPIO17_CTRL,
    GPIO17_IN,
    GPIO17_OUT,
    GPIO17_OUT_SET,
    GPIO17_OUT_CLR,
    GPIO17_OUT_XOR,
    GPIO17_OE,
    GPIO17_OE_SET,
    GPIO17_OE_CLR,
    GPIO17_OE_XOR,
}

map_gpio! {
    "Extracts GPIO18 register tokens.",
    periph_gpio18,
    "GPIO18 peripheral variant.",
    Gpio18,
    GPIO18_CTRL,
    GPIO18_IN,
    GPIO18_OUT,
    GPIO18_OUT_SET,
    GPIO18_OUT_CLR,
    GPIO18_OUT_XOR,
    GPIO18_OE,
    GPIO18_OE_SET,
    GPIO18_OE_CLR,
    GPIO18_OE_XOR,
}

map_gpio! {
    "Extracts GPIO19 register tokens.",
    periph_gpio19,
    "GPIO19 peripheral variant.",
    Gpio19,
    GPIO19_CTRL,
    GPIO19_IN,
    GPIO19_OUT,
    GPIO19_OUT_SET,
    GPIO19_OUT_CLR,
    GPIO19_OUT_XOR,
    GPIO19_OE,
    GPIO19_OE_SET,
    GPIO19_OE_CLR,
    GPIO19_OE_XOR,
}

map_gpio! {
    "Extracts GPIO20 register tokens.",
    periph_gpio20,
    "GPIO20 peripheral variant.",
    Gpio20,
    GPIO20_CTRL,
    GPIO20_IN,
    GPIO20_OUT,
    GPIO20_OUT_SET,
    GPIO20_OUT_CLR,
    GPIO20_OUT_XOR,
    GPIO20_OE,
    GPIO20_OE_SET,
    GPIO20_OE_CLR,
    GPIO20_OE_XOR,
}

map_gpio! {
    "Extracts GPIO21 register tokens.",
    periph_gpio21,
    "GPIO21 peripheral variant.",
    Gpio21,
    GPIO21_CTRL,
    GPIO21_IN,
    GPIO21_OUT,
    GPIO21_OUT_SET,
    GPIO21_OUT_CLR,
    GPIO21_OUT_XOR,
    GPIO21_OE,
    GPIO21_OE_SET,
    GPIO21_OE_CLR,
    GPIO21_OE_XOR,
}

map_gpio! {
    "Extracts GPIO22 register tokens.",
    periph_gpio22,
    "GPIO22 peripheral variant.",
    Gpio22,
    GPIO22_CTRL,
    GPIO22_IN,
    GPIO22_OUT,
    GPIO22_OUT_SET,
    GPIO22_OUT_CLR,
    GPIO22_OUT_XOR,
    GPIO22_OE,
    GPIO22_OE_SET,
    GPIO22_OE_CLR,
    GPIO22_OE_XOR,
}

map_gpio! {
    "Extracts GPIO23 register tokens.",
    periph_gpio23,
    "GPIO23 peripheral variant.",
    Gpio23,
    GPIO23_CTRL,
    GPIO23_IN,
    GPIO23_OUT,
    GPIO23_OUT_SET,
    GPIO23_OUT_CLR,
    GPIO23_OUT_XOR,
    GPIO23_OE,
    GPIO23_OE_SET,
    GPIO23_OE_CLR,
    GPIO23_OE_XOR,
}

map_gpio! {
    "Extracts GPIO24 register tokens.",
    periph_gpio24,
    "GPIO24 peripheral variant.",
    Gpio24,
    GPIO24_CTRL,
    GPIO24_IN,
    GPIO24_OUT,
    GPIO24_OUT_SET,
    GPIO24_OUT_CLR,
    GPIO24_OUT_XOR,
    GPIO24_OE,
    GPIO24_OE_SET,
    GPIO24_OE_CLR,
    GPIO24_OE_XOR,
}

map_gpio! {
    "Extracts GPIO25 register tokens.",
    periph_gpio25,
    "GPIO25 peripheral variant.",
    Gpio25,
    GPIO25_CTRL,
    GPIO25_IN,
    GPIO25_OUT,
    GPIO25_OUT_SET,
    GPIO25_OUT_CLR,
    GPIO25_OUT_XOR,
    GPIO25_OE,
    GPIO25_OE_SET,
    GPIO25_OE_CLR,
    GPIO25_OE_XOR,
}

map_gpio! {
    "Extracts GPIO26 register tokens.",
    periph_gpio26,
    "GPIO26 peripheral variant.",
    Gpio26,
    GPIO26_CTRL,
    GPIO26_IN,
    GPIO26_OUT,
    GPIO26_OUT_SET,
    GPIO26_OUT_CLR,
    GPIO26_OUT_XOR,
    GPIO26_OE,
    GPIO26_OE_SET,
    GPIO26_OE_CLR,
    GPIO26_OE_XOR,
}

map_gpio! {
    "Extracts GPIO27 register tokens.",
    periph_gpio27,
    "GPIO27 peripheral variant.",
    Gpio27,
    GPIO27_CTRL,
    GPIO27_IN,
    GPIO27_OUT,
    GPIO27_OUT_SET,
    GPIO27_OUT_CLR,
    GPIO27_OUT_XOR,
    GPIO27_OE,
    GPIO27_OE_SET,
    GPIO27_OE_CLR,
    GPIO27_OE_XOR,
}

map_gpio! {
    "Extracts GPIO28 register tokens.",
    periph_gpio28,
    "GPIO28 peripheral variant.",
    Gpio28,
    GPIO28_CTRL,
    GPIO28_IN,
    GPIO28_OUT,
    GPIO28_OUT_SET,
    GPIO28_OUT_CLR,
    GPIO28_OUT_XOR,
    GPIO28_OE,
    GPIO28_OE_SET,
    GPIO28_OE_CLR,
    GPIO28_OE_XOR,
}

map_gpio! {
    "Extracts GPIO29 register tokens.",
    periph_gpio29,
    "GPIO29 peripheral variant.",
    Gpio29,
    GPIO29_CTRL,
    GPIO29_IN,
    GPIO29_OUT,
    GPIO29_OUT_SET,
    GPIO29_OUT_CLR,
    GPIO29_OUT_XOR,
    GPIO29_OE,
    GPIO29_OE_SET,
    GPIO29_OE_CLR,
    GPIO29_OE_XOR,
}
