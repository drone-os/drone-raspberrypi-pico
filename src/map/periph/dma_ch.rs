//! Direct Memory Access (DMA) controller channels.

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic DMA channel peripheral variant.
    pub trait DmaChMap {}

    /// Generic DMA channel peripheral.
    pub struct DmaCh;

    DMA {
        CH_READ_ADDR {
            @AL0 0x20 RwRegAtomicAlias;
            @AL1 0x20 RwRegAtomicAlias;
            @AL2 0x20 RwRegAtomicAlias;
            @AL3_TRIG 0x20 RwRegAtomicAlias;
        }
        CH_WRITE_ADDR {
            @AL0 0x20 RwRegAtomicAlias;
            @AL1 0x20 RwRegAtomicAlias;
            @AL2_TRIG 0x20 RwRegAtomicAlias;
            @AL3 0x20 RwRegAtomicAlias;
        }
        CH_TRANS_COUNT {
            @AL0 0x20 RwRegAtomicAlias;
            @AL1_TRIG 0x20 RwRegAtomicAlias;
            @AL2 0x20 RwRegAtomicAlias;
            @AL3 0x20 RwRegAtomicAlias;
        }
        CH_CTRL {
            @AL0_TRIG 0x20 RwRegAtomicAlias;
            AHB_ERROR { RoRwRegFieldBit }
            READ_ERROR { RwRwRegFieldBit }
            WRITE_ERROR { RwRwRegFieldBit }
            BUSY { RoRwRegFieldBit }
            SNIFF_EN { RwRwRegFieldBit }
            BSWAP { RwRwRegFieldBit }
            IRQ_QUIET { RwRwRegFieldBit }
            TREQ_SEL { RwRwRegFieldBits }
            CHAIN_TO { RwRwRegFieldBits }
            RING_SEL { RwRwRegFieldBit }
            RING_SIZE { RwRwRegFieldBits }
            INCR_WRITE { RwRwRegFieldBit }
            INCR_READ { RwRwRegFieldBit }
            DATA_SIZE { RwRwRegFieldBits }
            HIGH_PRIORITY { RwRwRegFieldBit }
            EN { RwRwRegFieldBit }
            @AL1 0x20 RwRegAtomicAlias;
            AHB_ERROR { RoRwRegFieldBit }
            READ_ERROR { RwRwRegFieldBit }
            WRITE_ERROR { RwRwRegFieldBit }
            BUSY { RoRwRegFieldBit }
            SNIFF_EN { RwRwRegFieldBit }
            BSWAP { RwRwRegFieldBit }
            IRQ_QUIET { RwRwRegFieldBit }
            TREQ_SEL { RwRwRegFieldBits }
            CHAIN_TO { RwRwRegFieldBits }
            RING_SEL { RwRwRegFieldBit }
            RING_SIZE { RwRwRegFieldBits }
            INCR_WRITE { RwRwRegFieldBit }
            INCR_READ { RwRwRegFieldBit }
            DATA_SIZE { RwRwRegFieldBits }
            HIGH_PRIORITY { RwRwRegFieldBit }
            EN { RwRwRegFieldBit }
            @AL2 0x20 RwRegAtomicAlias;
            AHB_ERROR { RoRwRegFieldBit }
            READ_ERROR { RwRwRegFieldBit }
            WRITE_ERROR { RwRwRegFieldBit }
            BUSY { RoRwRegFieldBit }
            SNIFF_EN { RwRwRegFieldBit }
            BSWAP { RwRwRegFieldBit }
            IRQ_QUIET { RwRwRegFieldBit }
            TREQ_SEL { RwRwRegFieldBits }
            CHAIN_TO { RwRwRegFieldBits }
            RING_SEL { RwRwRegFieldBit }
            RING_SIZE { RwRwRegFieldBits }
            INCR_WRITE { RwRwRegFieldBit }
            INCR_READ { RwRwRegFieldBit }
            DATA_SIZE { RwRwRegFieldBits }
            HIGH_PRIORITY { RwRwRegFieldBit }
            EN { RwRwRegFieldBit }
            @AL3 0x20 RwRegAtomicAlias;
            AHB_ERROR { RoRwRegFieldBit }
            READ_ERROR { RwRwRegFieldBit }
            WRITE_ERROR { RwRwRegFieldBit }
            BUSY { RoRwRegFieldBit }
            SNIFF_EN { RwRwRegFieldBit }
            BSWAP { RwRwRegFieldBit }
            IRQ_QUIET { RwRwRegFieldBit }
            TREQ_SEL { RwRwRegFieldBits }
            CHAIN_TO { RwRwRegFieldBits }
            RING_SEL { RwRwRegFieldBit }
            RING_SIZE { RwRwRegFieldBits }
            INCR_WRITE { RwRwRegFieldBit }
            INCR_READ { RwRwRegFieldBit }
            DATA_SIZE { RwRwRegFieldBits }
            HIGH_PRIORITY { RwRwRegFieldBit }
            EN { RwRwRegFieldBit }
        }
        INTR {
            0x20 RoRegAtomicAlias Shared;
            INTR { RoRoRegFieldBit }
        }
        INTE0 {
            0x20 RwRegAtomicAlias Shared;
            INTE0 { RwRwRegFieldBit }
        }
        INTF0 {
            0x20 RwRegAtomicAlias Shared;
            INTF0 { RwRwRegFieldBit }
        }
        INTS0 {
            0x20 RwRegAtomicAlias Shared;
            INTS0 { RwRwRegFieldBit }
        }
        INTE1 {
            0x20 RwRegAtomicAlias Shared;
            INTE1 { RwRwRegFieldBit }
        }
        INTF1 {
            0x20 RwRegAtomicAlias Shared;
            INTF1 { RwRwRegFieldBit }
        }
        INTS1 {
            0x20 RwRegAtomicAlias Shared;
            INTS1 { RwRwRegFieldBit }
        }
        MULTI_CHAN_TRIGGER {
            0x20 WoRegAtomicAlias Shared;
            MULTI_CHAN_TRIGGER { WoWoRegFieldBit }
        }
        CHAN_ABORT {
            0x20 RwRegAtomicAlias Shared;
            CHAN_ABORT { RwRwRegFieldBit }
        }
        CH_DBG_CTDREQ {
            0x20 RwRegAtomicAlias;
            CH_DBG_CTDREQ { RwRwRegFieldBits }
        }
        CH_DBG_TCR {
            0x20 RoRegAtomicAlias;
        }
    }
}

macro_rules! map_dma_ch {
    (
        $dma_ch_macro_doc:expr,
        $dma_ch_macro:ident,
        $dma_ch_ty_doc:expr,
        $dma_ch_ty:ident,(
            $ch_read_addr_al0:ident,
            $ch_read_addr_al1:ident,
            $ch_read_addr_al2:ident,
            $ch_read_addr_al3_trig:ident
        ),(
            $ch_write_addr_al0:ident,
            $ch_write_addr_al1:ident,
            $ch_write_addr_al2_trig:ident,
            $ch_write_addr_al3:ident
        ),(
            $ch_trans_count_al0:ident,
            $ch_trans_count_al1_trig:ident,
            $ch_trans_count_al2:ident,
            $ch_trans_count_al3:ident
        ),($ch_ctrl_al0_trig:ident, $ch_ctrl_al1:ident, $ch_ctrl_al2:ident, $ch_ctrl_al3:ident),
        $intr:ident,
        $inte0:ident,
        $intf0:ident,
        $ints0:ident,
        $inte1:ident,
        $intf1:ident,
        $ints1:ident,
        $multi_chan_trigger:ident,
        $chan_abort:ident,
        $ch_dbg_ctdreq:ident,
        $ch_dbg_tcr:ident,
    ) => {
        periph::map! {
            #[doc = $dma_ch_macro_doc]
            pub macro $dma_ch_macro;

            #[doc = $dma_ch_ty_doc]
            pub struct $dma_ch_ty;

            impl DmaChMap for $dma_ch_ty {}

            crate::map::reg;
            crate::map::periph::dma_ch;

            DMA {
                CH_READ_ADDR {
                    @AL0 $ch_read_addr_al0;
                    @AL1 $ch_read_addr_al1;
                    @AL2 $ch_read_addr_al2;
                    @AL3_TRIG $ch_read_addr_al3_trig;
                }
                CH_WRITE_ADDR {
                    @AL0 $ch_write_addr_al0;
                    @AL1 $ch_write_addr_al1;
                    @AL2_TRIG $ch_write_addr_al2_trig;
                    @AL3 $ch_write_addr_al3;
                }
                CH_TRANS_COUNT {
                    @AL0 $ch_trans_count_al0;
                    @AL1_TRIG $ch_trans_count_al1_trig;
                    @AL2 $ch_trans_count_al2;
                    @AL3 $ch_trans_count_al3;
                }
                CH_CTRL {
                    @AL0_TRIG $ch_ctrl_al0_trig;
                    AHB_ERROR { AHB_ERROR }
                    READ_ERROR { READ_ERROR }
                    WRITE_ERROR { WRITE_ERROR }
                    BUSY { BUSY }
                    SNIFF_EN { SNIFF_EN }
                    BSWAP { BSWAP }
                    IRQ_QUIET { IRQ_QUIET }
                    TREQ_SEL { TREQ_SEL }
                    CHAIN_TO { CHAIN_TO }
                    RING_SEL { RING_SEL }
                    RING_SIZE { RING_SIZE }
                    INCR_WRITE { INCR_WRITE }
                    INCR_READ { INCR_READ }
                    DATA_SIZE { DATA_SIZE }
                    HIGH_PRIORITY { HIGH_PRIORITY }
                    EN { EN }
                    @AL1 $ch_ctrl_al1;
                    AHB_ERROR { AHB_ERROR }
                    READ_ERROR { READ_ERROR }
                    WRITE_ERROR { WRITE_ERROR }
                    BUSY { BUSY }
                    SNIFF_EN { SNIFF_EN }
                    BSWAP { BSWAP }
                    IRQ_QUIET { IRQ_QUIET }
                    TREQ_SEL { TREQ_SEL }
                    CHAIN_TO { CHAIN_TO }
                    RING_SEL { RING_SEL }
                    RING_SIZE { RING_SIZE }
                    INCR_WRITE { INCR_WRITE }
                    INCR_READ { INCR_READ }
                    DATA_SIZE { DATA_SIZE }
                    HIGH_PRIORITY { HIGH_PRIORITY }
                    EN { EN }
                    @AL2 $ch_ctrl_al2;
                    AHB_ERROR { AHB_ERROR }
                    READ_ERROR { READ_ERROR }
                    WRITE_ERROR { WRITE_ERROR }
                    BUSY { BUSY }
                    SNIFF_EN { SNIFF_EN }
                    BSWAP { BSWAP }
                    IRQ_QUIET { IRQ_QUIET }
                    TREQ_SEL { TREQ_SEL }
                    CHAIN_TO { CHAIN_TO }
                    RING_SEL { RING_SEL }
                    RING_SIZE { RING_SIZE }
                    INCR_WRITE { INCR_WRITE }
                    INCR_READ { INCR_READ }
                    DATA_SIZE { DATA_SIZE }
                    HIGH_PRIORITY { HIGH_PRIORITY }
                    EN { EN }
                    @AL3 $ch_ctrl_al3;
                    AHB_ERROR { AHB_ERROR }
                    READ_ERROR { READ_ERROR }
                    WRITE_ERROR { WRITE_ERROR }
                    BUSY { BUSY }
                    SNIFF_EN { SNIFF_EN }
                    BSWAP { BSWAP }
                    IRQ_QUIET { IRQ_QUIET }
                    TREQ_SEL { TREQ_SEL }
                    CHAIN_TO { CHAIN_TO }
                    RING_SEL { RING_SEL }
                    RING_SIZE { RING_SIZE }
                    INCR_WRITE { INCR_WRITE }
                    INCR_READ { INCR_READ }
                    DATA_SIZE { DATA_SIZE }
                    HIGH_PRIORITY { HIGH_PRIORITY }
                    EN { EN }
                }
                INTR {
                    INTR Shared;
                    INTR { $intr }
                }
                INTE0 {
                    INTE0 Shared;
                    INTE0 { $inte0 }
                }
                INTF0 {
                    INTF0 Shared;
                    INTF0 { $intf0 }
                }
                INTS0 {
                    INTS0 Shared;
                    INTS0 { $ints0 }
                }
                INTE1 {
                    INTE1 Shared;
                    INTE1 { $inte1 }
                }
                INTF1 {
                    INTF1 Shared;
                    INTF1 { $intf1 }
                }
                INTS1 {
                    INTS1 Shared;
                    INTS1 { $ints1 }
                }
                MULTI_CHAN_TRIGGER {
                    MULTI_CHAN_TRIGGER Shared;
                    MULTI_CHAN_TRIGGER { $multi_chan_trigger }
                }
                CHAN_ABORT {
                    CHAN_ABORT Shared;
                    CHAN_ABORT { $chan_abort }
                }
                CH_DBG_CTDREQ {
                    $ch_dbg_ctdreq;
                    CH_DBG_CTDREQ { $ch_dbg_ctdreq }
                }
                CH_DBG_TCR {
                    $ch_dbg_tcr;
                }
            }
        }
    };
}

map_dma_ch! {
    "Extracts DMA_CH0 register tokens.",
    periph_dma_ch0,
    "dma_ch0 peripheral variant.",
    DmaCh0,
    (CH0_READ_ADDR, CH0_AL1_READ_ADDR, CH0_AL2_READ_ADDR, CH0_AL3_READ_ADDR_TRIG),
    (CH0_WRITE_ADDR, CH0_AL1_WRITE_ADDR, CH0_AL2_WRITE_ADDR_TRIG, CH0_AL3_WRITE_ADDR),
    (CH0_TRANS_COUNT, CH0_AL1_TRANS_COUNT_TRIG, CH0_AL2_TRANS_COUNT, CH0_AL3_TRANS_COUNT),
    (CH0_CTRL_TRIG, CH0_AL1_CTRL, CH0_AL2_CTRL, CH0_AL3_CTRL),
    INTR0,
    INTE00,
    INTF00,
    INTS00,
    INTE10,
    INTF10,
    INTS10,
    MULTI_CHAN_TRIGGER0,
    CHAN_ABORT0,
    CH0_DBG_CTDREQ,
    CH0_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH1 register tokens.",
    periph_dma_ch1,
    "dma_ch1 peripheral variant.",
    DmaCh1,
    (CH1_READ_ADDR, CH1_AL1_READ_ADDR, CH1_AL2_READ_ADDR, CH1_AL3_READ_ADDR_TRIG),
    (CH1_WRITE_ADDR, CH1_AL1_WRITE_ADDR, CH1_AL2_WRITE_ADDR_TRIG, CH1_AL3_WRITE_ADDR),
    (CH1_TRANS_COUNT, CH1_AL1_TRANS_COUNT_TRIG, CH1_AL2_TRANS_COUNT, CH1_AL3_TRANS_COUNT),
    (CH1_CTRL_TRIG, CH1_AL1_CTRL, CH1_AL2_CTRL, CH1_AL3_CTRL),
    INTR1,
    INTE01,
    INTF01,
    INTS01,
    INTE11,
    INTF11,
    INTS11,
    MULTI_CHAN_TRIGGER1,
    CHAN_ABORT1,
    CH1_DBG_CTDREQ,
    CH1_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH2 register tokens.",
    periph_dma_ch2,
    "dma_ch2 peripheral variant.",
    DmaCh2,
    (CH2_READ_ADDR, CH2_AL1_READ_ADDR, CH2_AL2_READ_ADDR, CH2_AL3_READ_ADDR_TRIG),
    (CH2_WRITE_ADDR, CH2_AL1_WRITE_ADDR, CH2_AL2_WRITE_ADDR_TRIG, CH2_AL3_WRITE_ADDR),
    (CH2_TRANS_COUNT, CH2_AL1_TRANS_COUNT_TRIG, CH2_AL2_TRANS_COUNT, CH2_AL3_TRANS_COUNT),
    (CH2_CTRL_TRIG, CH2_AL1_CTRL, CH2_AL2_CTRL, CH2_AL3_CTRL),
    INTR2,
    INTE02,
    INTF02,
    INTS02,
    INTE12,
    INTF12,
    INTS12,
    MULTI_CHAN_TRIGGER2,
    CHAN_ABORT2,
    CH2_DBG_CTDREQ,
    CH2_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH3 register tokens.",
    periph_dma_ch3,
    "dma_ch3 peripheral variant.",
    DmaCh3,
    (CH3_READ_ADDR, CH3_AL1_READ_ADDR, CH3_AL2_READ_ADDR, CH3_AL3_READ_ADDR_TRIG),
    (CH3_WRITE_ADDR, CH3_AL1_WRITE_ADDR, CH3_AL2_WRITE_ADDR_TRIG, CH3_AL3_WRITE_ADDR),
    (CH3_TRANS_COUNT, CH3_AL1_TRANS_COUNT_TRIG, CH3_AL2_TRANS_COUNT, CH3_AL3_TRANS_COUNT),
    (CH3_CTRL_TRIG, CH3_AL1_CTRL, CH3_AL2_CTRL, CH3_AL3_CTRL),
    INTR3,
    INTE03,
    INTF03,
    INTS03,
    INTE13,
    INTF13,
    INTS13,
    MULTI_CHAN_TRIGGER3,
    CHAN_ABORT3,
    CH3_DBG_CTDREQ,
    CH3_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH4 register tokens.",
    periph_dma_ch4,
    "dma_ch4 peripheral variant.",
    DmaCh4,
    (CH4_READ_ADDR, CH4_AL1_READ_ADDR, CH4_AL2_READ_ADDR, CH4_AL3_READ_ADDR_TRIG),
    (CH4_WRITE_ADDR, CH4_AL1_WRITE_ADDR, CH4_AL2_WRITE_ADDR_TRIG, CH4_AL3_WRITE_ADDR),
    (CH4_TRANS_COUNT, CH4_AL1_TRANS_COUNT_TRIG, CH4_AL2_TRANS_COUNT, CH4_AL3_TRANS_COUNT),
    (CH4_CTRL_TRIG, CH4_AL1_CTRL, CH4_AL2_CTRL, CH4_AL3_CTRL),
    INTR4,
    INTE04,
    INTF04,
    INTS04,
    INTE14,
    INTF14,
    INTS14,
    MULTI_CHAN_TRIGGER4,
    CHAN_ABORT4,
    CH4_DBG_CTDREQ,
    CH4_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH5 register tokens.",
    periph_dma_ch5,
    "dma_ch5 peripheral variant.",
    DmaCh5,
    (CH5_READ_ADDR, CH5_AL1_READ_ADDR, CH5_AL2_READ_ADDR, CH5_AL3_READ_ADDR_TRIG),
    (CH5_WRITE_ADDR, CH5_AL1_WRITE_ADDR, CH5_AL2_WRITE_ADDR_TRIG, CH5_AL3_WRITE_ADDR),
    (CH5_TRANS_COUNT, CH5_AL1_TRANS_COUNT_TRIG, CH5_AL2_TRANS_COUNT, CH5_AL3_TRANS_COUNT),
    (CH5_CTRL_TRIG, CH5_AL1_CTRL, CH5_AL2_CTRL, CH5_AL3_CTRL),
    INTR5,
    INTE05,
    INTF05,
    INTS05,
    INTE15,
    INTF15,
    INTS15,
    MULTI_CHAN_TRIGGER5,
    CHAN_ABORT5,
    CH5_DBG_CTDREQ,
    CH5_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH6 register tokens.",
    periph_dma_ch6,
    "dma_ch6 peripheral variant.",
    DmaCh6,
    (CH6_READ_ADDR, CH6_AL1_READ_ADDR, CH6_AL2_READ_ADDR, CH6_AL3_READ_ADDR_TRIG),
    (CH6_WRITE_ADDR, CH6_AL1_WRITE_ADDR, CH6_AL2_WRITE_ADDR_TRIG, CH6_AL3_WRITE_ADDR),
    (CH6_TRANS_COUNT, CH6_AL1_TRANS_COUNT_TRIG, CH6_AL2_TRANS_COUNT, CH6_AL3_TRANS_COUNT),
    (CH6_CTRL_TRIG, CH6_AL1_CTRL, CH6_AL2_CTRL, CH6_AL3_CTRL),
    INTR6,
    INTE06,
    INTF06,
    INTS06,
    INTE16,
    INTF16,
    INTS16,
    MULTI_CHAN_TRIGGER6,
    CHAN_ABORT6,
    CH6_DBG_CTDREQ,
    CH6_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH7 register tokens.",
    periph_dma_ch7,
    "dma_ch7 peripheral variant.",
    DmaCh7,
    (CH7_READ_ADDR, CH7_AL1_READ_ADDR, CH7_AL2_READ_ADDR, CH7_AL3_READ_ADDR_TRIG),
    (CH7_WRITE_ADDR, CH7_AL1_WRITE_ADDR, CH7_AL2_WRITE_ADDR_TRIG, CH7_AL3_WRITE_ADDR),
    (CH7_TRANS_COUNT, CH7_AL1_TRANS_COUNT_TRIG, CH7_AL2_TRANS_COUNT, CH7_AL3_TRANS_COUNT),
    (CH7_CTRL_TRIG, CH7_AL1_CTRL, CH7_AL2_CTRL, CH7_AL3_CTRL),
    INTR7,
    INTE07,
    INTF07,
    INTS07,
    INTE17,
    INTF17,
    INTS17,
    MULTI_CHAN_TRIGGER7,
    CHAN_ABORT7,
    CH7_DBG_CTDREQ,
    CH7_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH8 register tokens.",
    periph_dma_ch8,
    "dma_ch8 peripheral variant.",
    DmaCh8,
    (CH8_READ_ADDR, CH8_AL1_READ_ADDR, CH8_AL2_READ_ADDR, CH8_AL3_READ_ADDR_TRIG),
    (CH8_WRITE_ADDR, CH8_AL1_WRITE_ADDR, CH8_AL2_WRITE_ADDR_TRIG, CH8_AL3_WRITE_ADDR),
    (CH8_TRANS_COUNT, CH8_AL1_TRANS_COUNT_TRIG, CH8_AL2_TRANS_COUNT, CH8_AL3_TRANS_COUNT),
    (CH8_CTRL_TRIG, CH8_AL1_CTRL, CH8_AL2_CTRL, CH8_AL3_CTRL),
    INTR8,
    INTE08,
    INTF08,
    INTS08,
    INTE18,
    INTF18,
    INTS18,
    MULTI_CHAN_TRIGGER8,
    CHAN_ABORT8,
    CH8_DBG_CTDREQ,
    CH8_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH9 register tokens.",
    periph_dma_ch9,
    "dma_ch9 peripheral variant.",
    DmaCh9,
    (CH9_READ_ADDR, CH9_AL1_READ_ADDR, CH9_AL2_READ_ADDR, CH9_AL3_READ_ADDR_TRIG),
    (CH9_WRITE_ADDR, CH9_AL1_WRITE_ADDR, CH9_AL2_WRITE_ADDR_TRIG, CH9_AL3_WRITE_ADDR),
    (CH9_TRANS_COUNT, CH9_AL1_TRANS_COUNT_TRIG, CH9_AL2_TRANS_COUNT, CH9_AL3_TRANS_COUNT),
    (CH9_CTRL_TRIG, CH9_AL1_CTRL, CH9_AL2_CTRL, CH9_AL3_CTRL),
    INTR9,
    INTE09,
    INTF09,
    INTS09,
    INTE19,
    INTF19,
    INTS19,
    MULTI_CHAN_TRIGGER9,
    CHAN_ABORT9,
    CH9_DBG_CTDREQ,
    CH9_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH10 register tokens.",
    periph_dma_ch10,
    "dma_ch10 peripheral variant.",
    DmaCh10,
    (CH10_READ_ADDR, CH10_AL1_READ_ADDR, CH10_AL2_READ_ADDR, CH10_AL3_READ_ADDR_TRIG),
    (CH10_WRITE_ADDR, CH10_AL1_WRITE_ADDR, CH10_AL2_WRITE_ADDR_TRIG, CH10_AL3_WRITE_ADDR),
    (CH10_TRANS_COUNT, CH10_AL1_TRANS_COUNT_TRIG, CH10_AL2_TRANS_COUNT, CH10_AL3_TRANS_COUNT),
    (CH10_CTRL_TRIG, CH10_AL1_CTRL, CH10_AL2_CTRL, CH10_AL3_CTRL),
    INTR10,
    INTE010,
    INTF010,
    INTS010,
    INTE110,
    INTF110,
    INTS110,
    MULTI_CHAN_TRIGGER10,
    CHAN_ABORT10,
    CH10_DBG_CTDREQ,
    CH10_DBG_TCR,
}

map_dma_ch! {
    "Extracts DMA_CH11 register tokens.",
    periph_dma_ch11,
    "dma_ch11 peripheral variant.",
    DmaCh11,
    (CH11_READ_ADDR, CH11_AL1_READ_ADDR, CH11_AL2_READ_ADDR, CH11_AL3_READ_ADDR_TRIG),
    (CH11_WRITE_ADDR, CH11_AL1_WRITE_ADDR, CH11_AL2_WRITE_ADDR_TRIG, CH11_AL3_WRITE_ADDR),
    (CH11_TRANS_COUNT, CH11_AL1_TRANS_COUNT_TRIG, CH11_AL2_TRANS_COUNT, CH11_AL3_TRANS_COUNT),
    (CH11_CTRL_TRIG, CH11_AL1_CTRL, CH11_AL2_CTRL, CH11_AL3_CTRL),
    INTR11,
    INTE011,
    INTF011,
    INTS011,
    INTE111,
    INTF111,
    INTS111,
    MULTI_CHAN_TRIGGER11,
    CHAN_ABORT11,
    CH11_DBG_CTDREQ,
    CH11_DBG_TCR,
}
