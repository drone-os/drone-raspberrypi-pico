//! General Purpose Input / Output (GPIO) pins.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic GPIO peripheral variant.
    pub trait GpioMap {}

    /// Generic GPIO peripheral.
    pub struct Gpio;

    IO {
        GPIO_STATUS {
            0x20 RoRegAtomicAlias;
            IRQTOPROC { RoRoRegFieldBit }
            IRQFROMPAD { RoRoRegFieldBit }
            INTOPERI { RoRoRegFieldBit }
            INFROMPAD { RoRoRegFieldBit }
            OETOPAD { RoRoRegFieldBit }
            OEFROMPERI { RoRoRegFieldBit }
            OUTTOPAD { RoRoRegFieldBit }
            OUTFROMPERI { RoRoRegFieldBit }
        }
        GPIO_CTRL {
            0x20 RwRegAtomicAlias;
            IRQOVER { RwRwRegFieldBits }
            INOVER { RwRwRegFieldBits }
            OEOVER { RwRwRegFieldBits }
            OUTOVER { RwRwRegFieldBits }
            FUNCSEL { RwRwRegFieldBits }
        }
        INTR {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RoRwRegFieldBit }
            GPIO_LEVEL_LOW { RoRwRegFieldBit }
        }
        PROC0_INTE {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        PROC0_INTF {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        PROC0_INTS {
            0x20 RoRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RoRoRegFieldBit }
            GPIO_EDGE_LOW { RoRoRegFieldBit }
            GPIO_LEVEL_HIGH { RoRoRegFieldBit }
            GPIO_LEVEL_LOW { RoRoRegFieldBit }
        }
        PROC1_INTE {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        PROC1_INTF {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        PROC1_INTS {
            0x20 RoRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RoRoRegFieldBit }
            GPIO_EDGE_LOW { RoRoRegFieldBit }
            GPIO_LEVEL_HIGH { RoRoRegFieldBit }
            GPIO_LEVEL_LOW { RoRoRegFieldBit }
        }
        DORMANT_WAKE_INTE {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        DORMANT_WAKE_INTF {
            0x20 RwRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RwRwRegFieldBit }
            GPIO_EDGE_LOW { RwRwRegFieldBit }
            GPIO_LEVEL_HIGH { RwRwRegFieldBit }
            GPIO_LEVEL_LOW { RwRwRegFieldBit }
        }
        DORMANT_WAKE_INTS {
            0x20 RoRegAtomicAlias Shared;
            GPIO_EDGE_HIGH { RoRoRegFieldBit }
            GPIO_EDGE_LOW { RoRoRegFieldBit }
            GPIO_LEVEL_HIGH { RoRoRegFieldBit }
            GPIO_LEVEL_LOW { RoRoRegFieldBit }
        }
    }

    PADS {
        GPIO {
            0x20 RwRegAtomicAlias;
            OD { RwRwRegFieldBit }
            IE { RwRwRegFieldBit }
            DRIVE { RwRwRegFieldBits }
            PUE { RwRwRegFieldBit }
            PDE { RwRwRegFieldBit }
            SCHMITT { RwRwRegFieldBit }
            SLEWFAST { RwRwRegFieldBit }
        }
    }

    SYSCFG {
        PROC_IN_SYNC_BYPASS {
            0x20 RwRegAtomicAlias Shared;
            PROC_IN_SYNC_BYPASS { RwRwRegFieldBit }
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
        $io:ident,
        $pads:ident,
        $gpio:ident,
        $gpio_status:ident,
        $gpio_ctrl:ident,(
            $intr:ident,
            $proc0_inte:ident,
            $proc0_intf:ident,
            $proc0_ints:ident,
            $proc1_inte:ident,
            $proc1_intf:ident,
            $proc1_ints:ident,
            $dormant_wake_inte:ident,
            $dormant_wake_intf:ident,
            $dormant_wake_ints:ident,
            $gpio_edge_high:ident,
            $gpio_edge_low:ident,
            $gpio_level_high:ident,
            $gpio_level_low:ident
        ),($proc_in_sync_bypass:ident, $proc_in_sync_bypass_field:ident),($gpio_in:ident, $gpio_in_field:ident),($gpio_out:ident, $gpio_out_field:ident),($gpio_out_set:ident, $gpio_out_set_field:ident),($gpio_out_clr:ident, $gpio_out_clr_field:ident),($gpio_out_xor:ident, $gpio_out_xor_field:ident),($gpio_oe:ident, $gpio_oe_field:ident),($gpio_oe_set:ident, $gpio_oe_set_field:ident),($gpio_oe_clr:ident, $gpio_oe_clr_field:ident),($gpio_oe_xor:ident, $gpio_oe_xor_field:ident),
    ) => {
        periph::map! {
            #[doc = $gpio_macro_doc]
            pub macro $gpio_macro;

            #[doc = $gpio_ty_doc]
            pub struct $gpio_ty;

            impl GpioMap for $gpio_ty {}

            crate::map::reg;
            crate::map::periph::gpio;

            IO {
                $io;
                GPIO_STATUS {
                    $gpio_status;
                    IRQTOPROC { IRQTOPROC }
                    IRQFROMPAD { IRQFROMPAD }
                    INTOPERI { INTOPERI }
                    INFROMPAD { INFROMPAD }
                    OETOPAD { OETOPAD }
                    OEFROMPERI { OEFROMPERI }
                    OUTTOPAD { OUTTOPAD }
                    OUTFROMPERI { OUTFROMPERI }
                }
                GPIO_CTRL {
                    $gpio_ctrl;
                    IRQOVER { IRQOVER }
                    INOVER { INOVER }
                    OEOVER { OEOVER }
                    OUTOVER { OUTOVER }
                    FUNCSEL { FUNCSEL }
                }
                INTR {
                    $intr Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC0_INTE {
                    $proc0_inte Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC0_INTF {
                    $proc0_intf Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC0_INTS {
                    $proc0_ints Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC1_INTE {
                    $proc1_inte Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC1_INTF {
                    $proc1_intf Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                PROC1_INTS {
                    $proc1_ints Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                DORMANT_WAKE_INTE {
                    $dormant_wake_inte Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                DORMANT_WAKE_INTF {
                    $dormant_wake_intf Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
                DORMANT_WAKE_INTS {
                    $dormant_wake_ints Shared;
                    GPIO_EDGE_HIGH { $gpio_edge_high }
                    GPIO_EDGE_LOW { $gpio_edge_low }
                    GPIO_LEVEL_HIGH { $gpio_level_high }
                    GPIO_LEVEL_LOW { $gpio_level_low }
                }
            }

            PADS {
                $pads;
                GPIO {
                    $gpio;
                    OD { OD }
                    IE { IE }
                    DRIVE { DRIVE }
                    PUE { PUE }
                    PDE { PDE }
                    SCHMITT { SCHMITT }
                    SLEWFAST { SLEWFAST }
                }
            }

            SYSCFG {
                PROC_IN_SYNC_BYPASS {
                    $proc_in_sync_bypass Shared;
                    PROC_IN_SYNC_BYPASS { $proc_in_sync_bypass_field }
                }
            }

            SIO {
                GPIO_IN {
                    $gpio_in Shared;
                    GPIO_IN { $gpio_in_field }
                }
                GPIO_OUT {
                    $gpio_out Shared;
                    GPIO_OUT { $gpio_out_field }
                }
                GPIO_OUT_SET {
                    $gpio_out_set Shared;
                    GPIO_OUT_SET { $gpio_out_set_field }
                }
                GPIO_OUT_CLR {
                    $gpio_out_clr Shared;
                    GPIO_OUT_CLR { $gpio_out_clr_field }
                }
                GPIO_OUT_XOR {
                    $gpio_out_xor Shared;
                    GPIO_OUT_XOR { $gpio_out_xor_field }
                }
                GPIO_OE {
                    $gpio_oe Shared;
                    GPIO_OE { $gpio_oe_field }
                }
                GPIO_OE_SET {
                    $gpio_oe_set Shared;
                    GPIO_OE_SET { $gpio_oe_set_field }
                }
                GPIO_OE_CLR {
                    $gpio_oe_clr Shared;
                    GPIO_OE_CLR { $gpio_oe_clr_field }
                }
                GPIO_OE_XOR {
                    $gpio_oe_xor Shared;
                    GPIO_OE_XOR { $gpio_oe_xor_field }
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
    IO_BANK0,
    PADS_BANK0,
    GPIO0,
    GPIO0_STATUS,
    GPIO0_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO0_EDGE_HIGH, GPIO0_EDGE_LOW, GPIO0_LEVEL_HIGH, GPIO0_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS0),
    (GPIO_IN, GPIO0_IN),
    (GPIO_OUT, GPIO0_OUT),
    (GPIO_OUT_SET, GPIO0_OUT_SET),
    (GPIO_OUT_CLR, GPIO0_OUT_CLR),
    (GPIO_OUT_XOR, GPIO0_OUT_XOR),
    (GPIO_OE, GPIO0_OE),
    (GPIO_OE_SET, GPIO0_OE_SET),
    (GPIO_OE_CLR, GPIO0_OE_CLR),
    (GPIO_OE_XOR, GPIO0_OE_XOR),
}

map_gpio! {
    "Extracts GPIO1 register tokens.",
    periph_gpio1,
    "GPIO1 peripheral variant.",
    Gpio1,
    IO_BANK0,
    PADS_BANK0,
    GPIO1,
    GPIO1_STATUS,
    GPIO1_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO1_EDGE_HIGH, GPIO1_EDGE_LOW, GPIO1_LEVEL_HIGH, GPIO1_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS1),
    (GPIO_IN, GPIO1_IN),
    (GPIO_OUT, GPIO1_OUT),
    (GPIO_OUT_SET, GPIO1_OUT_SET),
    (GPIO_OUT_CLR, GPIO1_OUT_CLR),
    (GPIO_OUT_XOR, GPIO1_OUT_XOR),
    (GPIO_OE, GPIO1_OE),
    (GPIO_OE_SET, GPIO1_OE_SET),
    (GPIO_OE_CLR, GPIO1_OE_CLR),
    (GPIO_OE_XOR, GPIO1_OE_XOR),
}

map_gpio! {
    "Extracts GPIO2 register tokens.",
    periph_gpio2,
    "GPIO2 peripheral variant.",
    Gpio2,
    IO_BANK0,
    PADS_BANK0,
    GPIO2,
    GPIO2_STATUS,
    GPIO2_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO2_EDGE_HIGH, GPIO2_EDGE_LOW, GPIO2_LEVEL_HIGH, GPIO2_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS2),
    (GPIO_IN, GPIO2_IN),
    (GPIO_OUT, GPIO2_OUT),
    (GPIO_OUT_SET, GPIO2_OUT_SET),
    (GPIO_OUT_CLR, GPIO2_OUT_CLR),
    (GPIO_OUT_XOR, GPIO2_OUT_XOR),
    (GPIO_OE, GPIO2_OE),
    (GPIO_OE_SET, GPIO2_OE_SET),
    (GPIO_OE_CLR, GPIO2_OE_CLR),
    (GPIO_OE_XOR, GPIO2_OE_XOR),
}

map_gpio! {
    "Extracts GPIO3 register tokens.",
    periph_gpio3,
    "GPIO3 peripheral variant.",
    Gpio3,
    IO_BANK0,
    PADS_BANK0,
    GPIO3,
    GPIO3_STATUS,
    GPIO3_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO3_EDGE_HIGH, GPIO3_EDGE_LOW, GPIO3_LEVEL_HIGH, GPIO3_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS3),
    (GPIO_IN, GPIO3_IN),
    (GPIO_OUT, GPIO3_OUT),
    (GPIO_OUT_SET, GPIO3_OUT_SET),
    (GPIO_OUT_CLR, GPIO3_OUT_CLR),
    (GPIO_OUT_XOR, GPIO3_OUT_XOR),
    (GPIO_OE, GPIO3_OE),
    (GPIO_OE_SET, GPIO3_OE_SET),
    (GPIO_OE_CLR, GPIO3_OE_CLR),
    (GPIO_OE_XOR, GPIO3_OE_XOR),
}

map_gpio! {
    "Extracts GPIO4 register tokens.",
    periph_gpio4,
    "GPIO4 peripheral variant.",
    Gpio4,
    IO_BANK0,
    PADS_BANK0,
    GPIO4,
    GPIO4_STATUS,
    GPIO4_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO4_EDGE_HIGH, GPIO4_EDGE_LOW, GPIO4_LEVEL_HIGH, GPIO4_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS4),
    (GPIO_IN, GPIO4_IN),
    (GPIO_OUT, GPIO4_OUT),
    (GPIO_OUT_SET, GPIO4_OUT_SET),
    (GPIO_OUT_CLR, GPIO4_OUT_CLR),
    (GPIO_OUT_XOR, GPIO4_OUT_XOR),
    (GPIO_OE, GPIO4_OE),
    (GPIO_OE_SET, GPIO4_OE_SET),
    (GPIO_OE_CLR, GPIO4_OE_CLR),
    (GPIO_OE_XOR, GPIO4_OE_XOR),
}

map_gpio! {
    "Extracts GPIO5 register tokens.",
    periph_gpio5,
    "GPIO5 peripheral variant.",
    Gpio5,
    IO_BANK0,
    PADS_BANK0,
    GPIO5,
    GPIO5_STATUS,
    GPIO5_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO5_EDGE_HIGH, GPIO5_EDGE_LOW, GPIO5_LEVEL_HIGH, GPIO5_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS5),
    (GPIO_IN, GPIO5_IN),
    (GPIO_OUT, GPIO5_OUT),
    (GPIO_OUT_SET, GPIO5_OUT_SET),
    (GPIO_OUT_CLR, GPIO5_OUT_CLR),
    (GPIO_OUT_XOR, GPIO5_OUT_XOR),
    (GPIO_OE, GPIO5_OE),
    (GPIO_OE_SET, GPIO5_OE_SET),
    (GPIO_OE_CLR, GPIO5_OE_CLR),
    (GPIO_OE_XOR, GPIO5_OE_XOR),
}

map_gpio! {
    "Extracts GPIO6 register tokens.",
    periph_gpio6,
    "GPIO6 peripheral variant.",
    Gpio6,
    IO_BANK0,
    PADS_BANK0,
    GPIO6,
    GPIO6_STATUS,
    GPIO6_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO6_EDGE_HIGH, GPIO6_EDGE_LOW, GPIO6_LEVEL_HIGH, GPIO6_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS6),
    (GPIO_IN, GPIO6_IN),
    (GPIO_OUT, GPIO6_OUT),
    (GPIO_OUT_SET, GPIO6_OUT_SET),
    (GPIO_OUT_CLR, GPIO6_OUT_CLR),
    (GPIO_OUT_XOR, GPIO6_OUT_XOR),
    (GPIO_OE, GPIO6_OE),
    (GPIO_OE_SET, GPIO6_OE_SET),
    (GPIO_OE_CLR, GPIO6_OE_CLR),
    (GPIO_OE_XOR, GPIO6_OE_XOR),
}

map_gpio! {
    "Extracts GPIO7 register tokens.",
    periph_gpio7,
    "GPIO7 peripheral variant.",
    Gpio7,
    IO_BANK0,
    PADS_BANK0,
    GPIO7,
    GPIO7_STATUS,
    GPIO7_CTRL,
    (
        INTR0, PROC0_INTE0, PROC0_INTF0, PROC0_INTS0, PROC1_INTE0, PROC1_INTF0, PROC1_INTS0,
        DORMANT_WAKE_INTE0, DORMANT_WAKE_INTF0, DORMANT_WAKE_INTS0,
        GPIO7_EDGE_HIGH, GPIO7_EDGE_LOW, GPIO7_LEVEL_HIGH, GPIO7_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS7),
    (GPIO_IN, GPIO7_IN),
    (GPIO_OUT, GPIO7_OUT),
    (GPIO_OUT_SET, GPIO7_OUT_SET),
    (GPIO_OUT_CLR, GPIO7_OUT_CLR),
    (GPIO_OUT_XOR, GPIO7_OUT_XOR),
    (GPIO_OE, GPIO7_OE),
    (GPIO_OE_SET, GPIO7_OE_SET),
    (GPIO_OE_CLR, GPIO7_OE_CLR),
    (GPIO_OE_XOR, GPIO7_OE_XOR),
}

map_gpio! {
    "Extracts GPIO8 register tokens.",
    periph_gpio8,
    "GPIO8 peripheral variant.",
    Gpio8,
    IO_BANK0,
    PADS_BANK0,
    GPIO8,
    GPIO8_STATUS,
    GPIO8_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO8_EDGE_HIGH, GPIO8_EDGE_LOW, GPIO8_LEVEL_HIGH, GPIO8_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS8),
    (GPIO_IN, GPIO8_IN),
    (GPIO_OUT, GPIO8_OUT),
    (GPIO_OUT_SET, GPIO8_OUT_SET),
    (GPIO_OUT_CLR, GPIO8_OUT_CLR),
    (GPIO_OUT_XOR, GPIO8_OUT_XOR),
    (GPIO_OE, GPIO8_OE),
    (GPIO_OE_SET, GPIO8_OE_SET),
    (GPIO_OE_CLR, GPIO8_OE_CLR),
    (GPIO_OE_XOR, GPIO8_OE_XOR),
}

map_gpio! {
    "Extracts GPIO9 register tokens.",
    periph_gpio9,
    "GPIO9 peripheral variant.",
    Gpio9,
    IO_BANK0,
    PADS_BANK0,
    GPIO9,
    GPIO9_STATUS,
    GPIO9_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO9_EDGE_HIGH, GPIO9_EDGE_LOW, GPIO9_LEVEL_HIGH, GPIO9_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS9),
    (GPIO_IN, GPIO9_IN),
    (GPIO_OUT, GPIO9_OUT),
    (GPIO_OUT_SET, GPIO9_OUT_SET),
    (GPIO_OUT_CLR, GPIO9_OUT_CLR),
    (GPIO_OUT_XOR, GPIO9_OUT_XOR),
    (GPIO_OE, GPIO9_OE),
    (GPIO_OE_SET, GPIO9_OE_SET),
    (GPIO_OE_CLR, GPIO9_OE_CLR),
    (GPIO_OE_XOR, GPIO9_OE_XOR),
}

map_gpio! {
    "Extracts GPIO10 register tokens.",
    periph_gpio10,
    "GPIO10 peripheral variant.",
    Gpio10,
    IO_BANK0,
    PADS_BANK0,
    GPIO10,
    GPIO10_STATUS,
    GPIO10_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO10_EDGE_HIGH, GPIO10_EDGE_LOW, GPIO10_LEVEL_HIGH, GPIO10_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS10),
    (GPIO_IN, GPIO10_IN),
    (GPIO_OUT, GPIO10_OUT),
    (GPIO_OUT_SET, GPIO10_OUT_SET),
    (GPIO_OUT_CLR, GPIO10_OUT_CLR),
    (GPIO_OUT_XOR, GPIO10_OUT_XOR),
    (GPIO_OE, GPIO10_OE),
    (GPIO_OE_SET, GPIO10_OE_SET),
    (GPIO_OE_CLR, GPIO10_OE_CLR),
    (GPIO_OE_XOR, GPIO10_OE_XOR),
}

map_gpio! {
    "Extracts GPIO11 register tokens.",
    periph_gpio11,
    "GPIO11 peripheral variant.",
    Gpio11,
    IO_BANK0,
    PADS_BANK0,
    GPIO11,
    GPIO11_STATUS,
    GPIO11_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO11_EDGE_HIGH, GPIO11_EDGE_LOW, GPIO11_LEVEL_HIGH, GPIO11_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS11),
    (GPIO_IN, GPIO11_IN),
    (GPIO_OUT, GPIO11_OUT),
    (GPIO_OUT_SET, GPIO11_OUT_SET),
    (GPIO_OUT_CLR, GPIO11_OUT_CLR),
    (GPIO_OUT_XOR, GPIO11_OUT_XOR),
    (GPIO_OE, GPIO11_OE),
    (GPIO_OE_SET, GPIO11_OE_SET),
    (GPIO_OE_CLR, GPIO11_OE_CLR),
    (GPIO_OE_XOR, GPIO11_OE_XOR),
}

map_gpio! {
    "Extracts GPIO12 register tokens.",
    periph_gpio12,
    "GPIO12 peripheral variant.",
    Gpio12,
    IO_BANK0,
    PADS_BANK0,
    GPIO12,
    GPIO12_STATUS,
    GPIO12_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO12_EDGE_HIGH, GPIO12_EDGE_LOW, GPIO12_LEVEL_HIGH, GPIO12_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS12),
    (GPIO_IN, GPIO12_IN),
    (GPIO_OUT, GPIO12_OUT),
    (GPIO_OUT_SET, GPIO12_OUT_SET),
    (GPIO_OUT_CLR, GPIO12_OUT_CLR),
    (GPIO_OUT_XOR, GPIO12_OUT_XOR),
    (GPIO_OE, GPIO12_OE),
    (GPIO_OE_SET, GPIO12_OE_SET),
    (GPIO_OE_CLR, GPIO12_OE_CLR),
    (GPIO_OE_XOR, GPIO12_OE_XOR),
}

map_gpio! {
    "Extracts GPIO13 register tokens.",
    periph_gpio13,
    "GPIO13 peripheral variant.",
    Gpio13,
    IO_BANK0,
    PADS_BANK0,
    GPIO13,
    GPIO13_STATUS,
    GPIO13_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO13_EDGE_HIGH, GPIO13_EDGE_LOW, GPIO13_LEVEL_HIGH, GPIO13_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS13),
    (GPIO_IN, GPIO13_IN),
    (GPIO_OUT, GPIO13_OUT),
    (GPIO_OUT_SET, GPIO13_OUT_SET),
    (GPIO_OUT_CLR, GPIO13_OUT_CLR),
    (GPIO_OUT_XOR, GPIO13_OUT_XOR),
    (GPIO_OE, GPIO13_OE),
    (GPIO_OE_SET, GPIO13_OE_SET),
    (GPIO_OE_CLR, GPIO13_OE_CLR),
    (GPIO_OE_XOR, GPIO13_OE_XOR),
}

map_gpio! {
    "Extracts GPIO14 register tokens.",
    periph_gpio14,
    "GPIO14 peripheral variant.",
    Gpio14,
    IO_BANK0,
    PADS_BANK0,
    GPIO14,
    GPIO14_STATUS,
    GPIO14_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO14_EDGE_HIGH, GPIO14_EDGE_LOW, GPIO14_LEVEL_HIGH, GPIO14_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS14),
    (GPIO_IN, GPIO14_IN),
    (GPIO_OUT, GPIO14_OUT),
    (GPIO_OUT_SET, GPIO14_OUT_SET),
    (GPIO_OUT_CLR, GPIO14_OUT_CLR),
    (GPIO_OUT_XOR, GPIO14_OUT_XOR),
    (GPIO_OE, GPIO14_OE),
    (GPIO_OE_SET, GPIO14_OE_SET),
    (GPIO_OE_CLR, GPIO14_OE_CLR),
    (GPIO_OE_XOR, GPIO14_OE_XOR),
}

map_gpio! {
    "Extracts GPIO15 register tokens.",
    periph_gpio15,
    "GPIO15 peripheral variant.",
    Gpio15,
    IO_BANK0,
    PADS_BANK0,
    GPIO15,
    GPIO15_STATUS,
    GPIO15_CTRL,
    (
        INTR1, PROC0_INTE1, PROC0_INTF1, PROC0_INTS1, PROC1_INTE1, PROC1_INTF1, PROC1_INTS1,
        DORMANT_WAKE_INTE1, DORMANT_WAKE_INTF1, DORMANT_WAKE_INTS1,
        GPIO15_EDGE_HIGH, GPIO15_EDGE_LOW, GPIO15_LEVEL_HIGH, GPIO15_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS15),
    (GPIO_IN, GPIO15_IN),
    (GPIO_OUT, GPIO15_OUT),
    (GPIO_OUT_SET, GPIO15_OUT_SET),
    (GPIO_OUT_CLR, GPIO15_OUT_CLR),
    (GPIO_OUT_XOR, GPIO15_OUT_XOR),
    (GPIO_OE, GPIO15_OE),
    (GPIO_OE_SET, GPIO15_OE_SET),
    (GPIO_OE_CLR, GPIO15_OE_CLR),
    (GPIO_OE_XOR, GPIO15_OE_XOR),
}

map_gpio! {
    "Extracts GPIO16 register tokens.",
    periph_gpio16,
    "GPIO16 peripheral variant.",
    Gpio16,
    IO_BANK0,
    PADS_BANK0,
    GPIO16,
    GPIO16_STATUS,
    GPIO16_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO16_EDGE_HIGH, GPIO16_EDGE_LOW, GPIO16_LEVEL_HIGH, GPIO16_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS16),
    (GPIO_IN, GPIO16_IN),
    (GPIO_OUT, GPIO16_OUT),
    (GPIO_OUT_SET, GPIO16_OUT_SET),
    (GPIO_OUT_CLR, GPIO16_OUT_CLR),
    (GPIO_OUT_XOR, GPIO16_OUT_XOR),
    (GPIO_OE, GPIO16_OE),
    (GPIO_OE_SET, GPIO16_OE_SET),
    (GPIO_OE_CLR, GPIO16_OE_CLR),
    (GPIO_OE_XOR, GPIO16_OE_XOR),
}

map_gpio! {
    "Extracts GPIO17 register tokens.",
    periph_gpio17,
    "GPIO17 peripheral variant.",
    Gpio17,
    IO_BANK0,
    PADS_BANK0,
    GPIO17,
    GPIO17_STATUS,
    GPIO17_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO17_EDGE_HIGH, GPIO17_EDGE_LOW, GPIO17_LEVEL_HIGH, GPIO17_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS17),
    (GPIO_IN, GPIO17_IN),
    (GPIO_OUT, GPIO17_OUT),
    (GPIO_OUT_SET, GPIO17_OUT_SET),
    (GPIO_OUT_CLR, GPIO17_OUT_CLR),
    (GPIO_OUT_XOR, GPIO17_OUT_XOR),
    (GPIO_OE, GPIO17_OE),
    (GPIO_OE_SET, GPIO17_OE_SET),
    (GPIO_OE_CLR, GPIO17_OE_CLR),
    (GPIO_OE_XOR, GPIO17_OE_XOR),
}

map_gpio! {
    "Extracts GPIO18 register tokens.",
    periph_gpio18,
    "GPIO18 peripheral variant.",
    Gpio18,
    IO_BANK0,
    PADS_BANK0,
    GPIO18,
    GPIO18_STATUS,
    GPIO18_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO18_EDGE_HIGH, GPIO18_EDGE_LOW, GPIO18_LEVEL_HIGH, GPIO18_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS18),
    (GPIO_IN, GPIO18_IN),
    (GPIO_OUT, GPIO18_OUT),
    (GPIO_OUT_SET, GPIO18_OUT_SET),
    (GPIO_OUT_CLR, GPIO18_OUT_CLR),
    (GPIO_OUT_XOR, GPIO18_OUT_XOR),
    (GPIO_OE, GPIO18_OE),
    (GPIO_OE_SET, GPIO18_OE_SET),
    (GPIO_OE_CLR, GPIO18_OE_CLR),
    (GPIO_OE_XOR, GPIO18_OE_XOR),
}

map_gpio! {
    "Extracts GPIO19 register tokens.",
    periph_gpio19,
    "GPIO19 peripheral variant.",
    Gpio19,
    IO_BANK0,
    PADS_BANK0,
    GPIO19,
    GPIO19_STATUS,
    GPIO19_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO19_EDGE_HIGH, GPIO19_EDGE_LOW, GPIO19_LEVEL_HIGH, GPIO19_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS19),
    (GPIO_IN, GPIO19_IN),
    (GPIO_OUT, GPIO19_OUT),
    (GPIO_OUT_SET, GPIO19_OUT_SET),
    (GPIO_OUT_CLR, GPIO19_OUT_CLR),
    (GPIO_OUT_XOR, GPIO19_OUT_XOR),
    (GPIO_OE, GPIO19_OE),
    (GPIO_OE_SET, GPIO19_OE_SET),
    (GPIO_OE_CLR, GPIO19_OE_CLR),
    (GPIO_OE_XOR, GPIO19_OE_XOR),
}

map_gpio! {
    "Extracts GPIO20 register tokens.",
    periph_gpio20,
    "GPIO20 peripheral variant.",
    Gpio20,
    IO_BANK0,
    PADS_BANK0,
    GPIO20,
    GPIO20_STATUS,
    GPIO20_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO20_EDGE_HIGH, GPIO20_EDGE_LOW, GPIO20_LEVEL_HIGH, GPIO20_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS20),
    (GPIO_IN, GPIO20_IN),
    (GPIO_OUT, GPIO20_OUT),
    (GPIO_OUT_SET, GPIO20_OUT_SET),
    (GPIO_OUT_CLR, GPIO20_OUT_CLR),
    (GPIO_OUT_XOR, GPIO20_OUT_XOR),
    (GPIO_OE, GPIO20_OE),
    (GPIO_OE_SET, GPIO20_OE_SET),
    (GPIO_OE_CLR, GPIO20_OE_CLR),
    (GPIO_OE_XOR, GPIO20_OE_XOR),
}

map_gpio! {
    "Extracts GPIO21 register tokens.",
    periph_gpio21,
    "GPIO21 peripheral variant.",
    Gpio21,
    IO_BANK0,
    PADS_BANK0,
    GPIO21,
    GPIO21_STATUS,
    GPIO21_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO21_EDGE_HIGH, GPIO21_EDGE_LOW, GPIO21_LEVEL_HIGH, GPIO21_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS21),
    (GPIO_IN, GPIO21_IN),
    (GPIO_OUT, GPIO21_OUT),
    (GPIO_OUT_SET, GPIO21_OUT_SET),
    (GPIO_OUT_CLR, GPIO21_OUT_CLR),
    (GPIO_OUT_XOR, GPIO21_OUT_XOR),
    (GPIO_OE, GPIO21_OE),
    (GPIO_OE_SET, GPIO21_OE_SET),
    (GPIO_OE_CLR, GPIO21_OE_CLR),
    (GPIO_OE_XOR, GPIO21_OE_XOR),
}

map_gpio! {
    "Extracts GPIO22 register tokens.",
    periph_gpio22,
    "GPIO22 peripheral variant.",
    Gpio22,
    IO_BANK0,
    PADS_BANK0,
    GPIO22,
    GPIO22_STATUS,
    GPIO22_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO22_EDGE_HIGH, GPIO22_EDGE_LOW, GPIO22_LEVEL_HIGH, GPIO22_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS22),
    (GPIO_IN, GPIO22_IN),
    (GPIO_OUT, GPIO22_OUT),
    (GPIO_OUT_SET, GPIO22_OUT_SET),
    (GPIO_OUT_CLR, GPIO22_OUT_CLR),
    (GPIO_OUT_XOR, GPIO22_OUT_XOR),
    (GPIO_OE, GPIO22_OE),
    (GPIO_OE_SET, GPIO22_OE_SET),
    (GPIO_OE_CLR, GPIO22_OE_CLR),
    (GPIO_OE_XOR, GPIO22_OE_XOR),
}

map_gpio! {
    "Extracts GPIO23 register tokens.",
    periph_gpio23,
    "GPIO23 peripheral variant.",
    Gpio23,
    IO_BANK0,
    PADS_BANK0,
    GPIO23,
    GPIO23_STATUS,
    GPIO23_CTRL,
    (
        INTR2, PROC0_INTE2, PROC0_INTF2, PROC0_INTS2, PROC1_INTE2, PROC1_INTF2, PROC1_INTS2,
        DORMANT_WAKE_INTE2, DORMANT_WAKE_INTF2, DORMANT_WAKE_INTS2,
        GPIO23_EDGE_HIGH, GPIO23_EDGE_LOW, GPIO23_LEVEL_HIGH, GPIO23_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS23),
    (GPIO_IN, GPIO23_IN),
    (GPIO_OUT, GPIO23_OUT),
    (GPIO_OUT_SET, GPIO23_OUT_SET),
    (GPIO_OUT_CLR, GPIO23_OUT_CLR),
    (GPIO_OUT_XOR, GPIO23_OUT_XOR),
    (GPIO_OE, GPIO23_OE),
    (GPIO_OE_SET, GPIO23_OE_SET),
    (GPIO_OE_CLR, GPIO23_OE_CLR),
    (GPIO_OE_XOR, GPIO23_OE_XOR),
}

map_gpio! {
    "Extracts GPIO24 register tokens.",
    periph_gpio24,
    "GPIO24 peripheral variant.",
    Gpio24,
    IO_BANK0,
    PADS_BANK0,
    GPIO24,
    GPIO24_STATUS,
    GPIO24_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO24_EDGE_HIGH, GPIO24_EDGE_LOW, GPIO24_LEVEL_HIGH, GPIO24_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS24),
    (GPIO_IN, GPIO24_IN),
    (GPIO_OUT, GPIO24_OUT),
    (GPIO_OUT_SET, GPIO24_OUT_SET),
    (GPIO_OUT_CLR, GPIO24_OUT_CLR),
    (GPIO_OUT_XOR, GPIO24_OUT_XOR),
    (GPIO_OE, GPIO24_OE),
    (GPIO_OE_SET, GPIO24_OE_SET),
    (GPIO_OE_CLR, GPIO24_OE_CLR),
    (GPIO_OE_XOR, GPIO24_OE_XOR),
}

map_gpio! {
    "Extracts GPIO25 register tokens.",
    periph_gpio25,
    "GPIO25 peripheral variant.",
    Gpio25,
    IO_BANK0,
    PADS_BANK0,
    GPIO25,
    GPIO25_STATUS,
    GPIO25_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO25_EDGE_HIGH, GPIO25_EDGE_LOW, GPIO25_LEVEL_HIGH, GPIO25_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS25),
    (GPIO_IN, GPIO25_IN),
    (GPIO_OUT, GPIO25_OUT),
    (GPIO_OUT_SET, GPIO25_OUT_SET),
    (GPIO_OUT_CLR, GPIO25_OUT_CLR),
    (GPIO_OUT_XOR, GPIO25_OUT_XOR),
    (GPIO_OE, GPIO25_OE),
    (GPIO_OE_SET, GPIO25_OE_SET),
    (GPIO_OE_CLR, GPIO25_OE_CLR),
    (GPIO_OE_XOR, GPIO25_OE_XOR),
}

map_gpio! {
    "Extracts GPIO26 register tokens.",
    periph_gpio26,
    "GPIO26 peripheral variant.",
    Gpio26,
    IO_BANK0,
    PADS_BANK0,
    GPIO26,
    GPIO26_STATUS,
    GPIO26_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO26_EDGE_HIGH, GPIO26_EDGE_LOW, GPIO26_LEVEL_HIGH, GPIO26_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS26),
    (GPIO_IN, GPIO26_IN),
    (GPIO_OUT, GPIO26_OUT),
    (GPIO_OUT_SET, GPIO26_OUT_SET),
    (GPIO_OUT_CLR, GPIO26_OUT_CLR),
    (GPIO_OUT_XOR, GPIO26_OUT_XOR),
    (GPIO_OE, GPIO26_OE),
    (GPIO_OE_SET, GPIO26_OE_SET),
    (GPIO_OE_CLR, GPIO26_OE_CLR),
    (GPIO_OE_XOR, GPIO26_OE_XOR),
}

map_gpio! {
    "Extracts GPIO27 register tokens.",
    periph_gpio27,
    "GPIO27 peripheral variant.",
    Gpio27,
    IO_BANK0,
    PADS_BANK0,
    GPIO27,
    GPIO27_STATUS,
    GPIO27_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO27_EDGE_HIGH, GPIO27_EDGE_LOW, GPIO27_LEVEL_HIGH, GPIO27_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS27),
    (GPIO_IN, GPIO27_IN),
    (GPIO_OUT, GPIO27_OUT),
    (GPIO_OUT_SET, GPIO27_OUT_SET),
    (GPIO_OUT_CLR, GPIO27_OUT_CLR),
    (GPIO_OUT_XOR, GPIO27_OUT_XOR),
    (GPIO_OE, GPIO27_OE),
    (GPIO_OE_SET, GPIO27_OE_SET),
    (GPIO_OE_CLR, GPIO27_OE_CLR),
    (GPIO_OE_XOR, GPIO27_OE_XOR),
}

map_gpio! {
    "Extracts GPIO28 register tokens.",
    periph_gpio28,
    "GPIO28 peripheral variant.",
    Gpio28,
    IO_BANK0,
    PADS_BANK0,
    GPIO28,
    GPIO28_STATUS,
    GPIO28_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO28_EDGE_HIGH, GPIO28_EDGE_LOW, GPIO28_LEVEL_HIGH, GPIO28_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS28),
    (GPIO_IN, GPIO28_IN),
    (GPIO_OUT, GPIO28_OUT),
    (GPIO_OUT_SET, GPIO28_OUT_SET),
    (GPIO_OUT_CLR, GPIO28_OUT_CLR),
    (GPIO_OUT_XOR, GPIO28_OUT_XOR),
    (GPIO_OE, GPIO28_OE),
    (GPIO_OE_SET, GPIO28_OE_SET),
    (GPIO_OE_CLR, GPIO28_OE_CLR),
    (GPIO_OE_XOR, GPIO28_OE_XOR),
}

map_gpio! {
    "Extracts GPIO29 register tokens.",
    periph_gpio29,
    "GPIO29 peripheral variant.",
    Gpio29,
    IO_BANK0,
    PADS_BANK0,
    GPIO29,
    GPIO29_STATUS,
    GPIO29_CTRL,
    (
        INTR3, PROC0_INTE3, PROC0_INTF3, PROC0_INTS3, PROC1_INTE3, PROC1_INTF3, PROC1_INTS3,
        DORMANT_WAKE_INTE3, DORMANT_WAKE_INTF3, DORMANT_WAKE_INTS3,
        GPIO29_EDGE_HIGH, GPIO29_EDGE_LOW, GPIO29_LEVEL_HIGH, GPIO29_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS, PROC_IN_SYNC_BYPASS29),
    (GPIO_IN, GPIO29_IN),
    (GPIO_OUT, GPIO29_OUT),
    (GPIO_OUT_SET, GPIO29_OUT_SET),
    (GPIO_OUT_CLR, GPIO29_OUT_CLR),
    (GPIO_OUT_XOR, GPIO29_OUT_XOR),
    (GPIO_OE, GPIO29_OE),
    (GPIO_OE_SET, GPIO29_OE_SET),
    (GPIO_OE_CLR, GPIO29_OE_CLR),
    (GPIO_OE_XOR, GPIO29_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SCLK register tokens.",
    periph_gpio_qspi_sclk,
    "GPIO_QSPI_SCLK peripheral variant.",
    QspiSclk,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SCLK,
    GPIO_QSPI_SCLK_STATUS,
    GPIO_QSPI_SCLK_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SCLK_EDGE_HIGH, GPIO_QSPI_SCLK_EDGE_LOW, GPIO_QSPI_SCLK_LEVEL_HIGH, GPIO_QSPI_SCLK_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SCLK),
    (GPIO_HI_IN, GPIO_SCLK_HI_IN),
    (GPIO_HI_OUT, GPIO_SCLK_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SCLK_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SCLK_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SCLK_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SCLK_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SCLK_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SCLK_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SCLK_HI_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SS register tokens.",
    periph_gpio_qspi_ss,
    "GPIO_QSPI_SS peripheral variant.",
    QspiSs,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SS,
    GPIO_QSPI_SS_STATUS,
    GPIO_QSPI_SS_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SS_EDGE_HIGH, GPIO_QSPI_SS_EDGE_LOW, GPIO_QSPI_SS_LEVEL_HIGH, GPIO_QSPI_SS_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SS),
    (GPIO_HI_IN, GPIO_SS_HI_IN),
    (GPIO_HI_OUT, GPIO_SS_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SS_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SS_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SS_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SS_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SS_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SS_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SS_HI_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SD0 register tokens.",
    periph_gpio_qspi_sd0,
    "GPIO_QSPI_SD0 peripheral variant.",
    QspiSd0,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SD0,
    GPIO_QSPI_SD0_STATUS,
    GPIO_QSPI_SD0_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SD0_EDGE_HIGH, GPIO_QSPI_SD0_EDGE_LOW, GPIO_QSPI_SD0_LEVEL_HIGH, GPIO_QSPI_SD0_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SD0),
    (GPIO_HI_IN, GPIO_SD0_HI_IN),
    (GPIO_HI_OUT, GPIO_SD0_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SD0_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SD0_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SD0_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SD0_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SD0_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SD0_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SD0_HI_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SD1 register tokens.",
    periph_gpio_qspi_sd1,
    "GPIO_QSPI_SD1 peripheral variant.",
    QspiSd1,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SD1,
    GPIO_QSPI_SD1_STATUS,
    GPIO_QSPI_SD1_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SD1_EDGE_HIGH, GPIO_QSPI_SD1_EDGE_LOW, GPIO_QSPI_SD1_LEVEL_HIGH, GPIO_QSPI_SD1_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SD1),
    (GPIO_HI_IN, GPIO_SD1_HI_IN),
    (GPIO_HI_OUT, GPIO_SD1_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SD1_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SD1_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SD1_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SD1_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SD1_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SD1_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SD1_HI_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SD2 register tokens.",
    periph_gpio_qspi_sd2,
    "GPIO_QSPI_SD2 peripheral variant.",
    QspiSd2,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SD2,
    GPIO_QSPI_SD2_STATUS,
    GPIO_QSPI_SD2_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SD2_EDGE_HIGH, GPIO_QSPI_SD2_EDGE_LOW, GPIO_QSPI_SD2_LEVEL_HIGH, GPIO_QSPI_SD2_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SD2),
    (GPIO_HI_IN, GPIO_SD2_HI_IN),
    (GPIO_HI_OUT, GPIO_SD2_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SD2_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SD2_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SD2_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SD2_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SD2_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SD2_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SD2_HI_OE_XOR),
}

map_gpio! {
    "Extracts GPIO_QSPI_SD3 register tokens.",
    periph_gpio_qspi_sd3,
    "GPIO_QSPI_SD3 peripheral variant.",
    QspiSd3,
    IO_QSPI,
    PADS_QSPI,
    GPIO_QSPI_SD3,
    GPIO_QSPI_SD3_STATUS,
    GPIO_QSPI_SD3_CTRL,
    (
        INTR, PROC0_INTE, PROC0_INTF, PROC0_INTS, PROC1_INTE, PROC1_INTF, PROC1_INTS,
        DORMANT_WAKE_INTE, DORMANT_WAKE_INTF, DORMANT_WAKE_INTS,
        GPIO_QSPI_SD3_EDGE_HIGH, GPIO_QSPI_SD3_EDGE_LOW, GPIO_QSPI_SD3_LEVEL_HIGH, GPIO_QSPI_SD3_LEVEL_LOW
    ),
    (PROC_IN_SYNC_BYPASS_HI, PROC_IN_SYNC_BYPASS_HI_SD3),
    (GPIO_HI_IN, GPIO_SD3_HI_IN),
    (GPIO_HI_OUT, GPIO_SD3_HI_OUT),
    (GPIO_HI_OUT_SET, GPIO_SD3_HI_OUT_SET),
    (GPIO_HI_OUT_CLR, GPIO_SD3_HI_OUT_CLR),
    (GPIO_HI_OUT_XOR, GPIO_SD3_HI_OUT_XOR),
    (GPIO_HI_OE, GPIO_SD3_HI_OE),
    (GPIO_HI_OE_SET, GPIO_SD3_HI_OE_SET),
    (GPIO_HI_OE_CLR, GPIO_SD3_HI_OE_CLR),
    (GPIO_HI_OE_XOR, GPIO_SD3_HI_OE_XOR),
}
