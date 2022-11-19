//! Serial Peripheral Interface (SPI).

use crate::reg::marker::*;
use drone_core::periph;

periph! {
    /// Generic SPI peripheral variant.
    pub trait SpiMap {}

    /// Generic SPI peripheral.
    pub struct Spi;

    SPI {
        SSPCR0 {
            0x20 RwRegAtomicAlias;
            SCR { RwRwRegFieldBits }
            SPH { RwRwRegFieldBit }
            SPO { RwRwRegFieldBit }
            FRF { RwRwRegFieldBits }
            DSS { RwRwRegFieldBits }
        }
        SSPCR1 {
            0x20 RwRegAtomicAlias;
            SOD { RwRwRegFieldBit }
            MS { RwRwRegFieldBit }
            SSE { RwRwRegFieldBit }
            LBM { RwRwRegFieldBit }
        }
        SSPDR {
            0x20 RwRegAtomicAlias;
            DATA { RwRwRegFieldBits }
        }
        SSPSR {
            0x20 RoRegAtomicAlias;
            BSY { RoRoRegFieldBit }
            RFF { RoRoRegFieldBit }
            RNE { RoRoRegFieldBit }
            TNF { RoRoRegFieldBit }
            TFE { RoRoRegFieldBit }
        }
        SSPCPSR {
            0x20 RwRegAtomicAlias;
            CPSDVSR { RwRwRegFieldBits }
        }
        SSPIMSC {
            0x20 RwRegAtomicAlias;
            TXIM { RwRwRegFieldBit }
            RXIM { RwRwRegFieldBit }
            RTIM { RwRwRegFieldBit }
            RORIM { RwRwRegFieldBit }
        }
        SSPRIS {
            0x20 RoRegAtomicAlias;
            TXRIS { RoRoRegFieldBit }
            RXRIS { RoRoRegFieldBit }
            RTRIS { RoRoRegFieldBit }
            RORRIS { RoRoRegFieldBit }
        }
        SSPMIS {
            0x20 RoRegAtomicAlias;
            TXMIS { RoRoRegFieldBit }
            RXMIS { RoRoRegFieldBit }
            RTMIS { RoRoRegFieldBit }
            RORMIS { RoRoRegFieldBit }
        }
        SSPICR {
            0x20 WoRegAtomicAlias;
            RTIC { WoWoRegFieldBit }
            RORIC { WoWoRegFieldBit }
        }
        SSPDMACR {
            0x20 RwRegAtomicAlias;
            TXDMAE { RwRwRegFieldBit }
            RXDMAE { RwRwRegFieldBit }
        }
        SSPPERIPHID0 {
            0x20 RoRegAtomicAlias;
            PARTNUMBER0 { RoRoRegFieldBits }
        }
        SSPPERIPHID1 {
            0x20 RoRegAtomicAlias;
            DESIGNER0 { RoRoRegFieldBits }
            PARTNUMBER1 { RoRoRegFieldBits }
        }
        SSPPERIPHID2 {
            0x20 RoRegAtomicAlias;
            REVISION { RoRoRegFieldBits }
            DESIGNER1 { RoRoRegFieldBits }
        }
        SSPPERIPHID3 {
            0x20 RoRegAtomicAlias;
            CONFIGURATION { RoRoRegFieldBits }
        }
        SSPPCELLID0 {
            0x20 RoRegAtomicAlias;
            SSPPCELLID0 { RoRoRegFieldBits }
        }
        SSPPCELLID1 {
            0x20 RoRegAtomicAlias;
            SSPPCELLID1 { RoRoRegFieldBits }
        }
        SSPPCELLID2 {
            0x20 RoRegAtomicAlias;
            SSPPCELLID2 { RoRoRegFieldBits }
        }
        SSPPCELLID3 {
            0x20 RoRegAtomicAlias;
            SSPPCELLID3 { RoRoRegFieldBits }
        }
    }
}

macro_rules! map_spi {
    ($spi_macro_doc:expr, $spi_macro:ident, $spi_ty_doc:expr, $spi_ty:ident, $spi:ident,) => {
        periph::map! {
            #[doc = $spi_macro_doc]
            pub macro $spi_macro;

            #[doc = $spi_ty_doc]
            pub struct $spi_ty;

            impl SpiMap for $spi_ty {}

            crate::map::reg;
            crate::map::periph::spi;

            SPI {
                $spi;
                SSPCR0 {
                    SSPCR0;
                    SCR { SCR }
                    SPH { SPH }
                    SPO { SPO }
                    FRF { FRF }
                    DSS { DSS }
                }
                SSPCR1 {
                    SSPCR1;
                    SOD { SOD }
                    MS { MS }
                    SSE { SSE }
                    LBM { LBM }
                }
                SSPDR {
                    SSPDR;
                    DATA { DATA }
                }
                SSPSR {
                    SSPSR;
                    BSY { BSY }
                    RFF { RFF }
                    RNE { RNE }
                    TNF { TNF }
                    TFE { TFE }
                }
                SSPCPSR {
                    SSPCPSR;
                    CPSDVSR { CPSDVSR }
                }
                SSPIMSC {
                    SSPIMSC;
                    TXIM { TXIM }
                    RXIM { RXIM }
                    RTIM { RTIM }
                    RORIM { RORIM }
                }
                SSPRIS {
                    SSPRIS;
                    TXRIS { TXRIS }
                    RXRIS { RXRIS }
                    RTRIS { RTRIS }
                    RORRIS { RORRIS }
                }
                SSPMIS {
                    SSPMIS;
                    TXMIS { TXMIS }
                    RXMIS { RXMIS }
                    RTMIS { RTMIS }
                    RORMIS { RORMIS }
                }
                SSPICR {
                    SSPICR;
                    RTIC { RTIC }
                    RORIC { RORIC }
                }
                SSPDMACR {
                    SSPDMACR;
                    TXDMAE { TXDMAE }
                    RXDMAE { RXDMAE }
                }
                SSPPERIPHID0 {
                    SSPPERIPHID0;
                    PARTNUMBER0 { PARTNUMBER0 }
                }
                SSPPERIPHID1 {
                    SSPPERIPHID1;
                    DESIGNER0 { DESIGNER0 }
                    PARTNUMBER1 { PARTNUMBER1 }
                }
                SSPPERIPHID2 {
                    SSPPERIPHID2;
                    REVISION { REVISION }
                    DESIGNER1 { DESIGNER1 }
                }
                SSPPERIPHID3 {
                    SSPPERIPHID3;
                    CONFIGURATION { CONFIGURATION }
                }
                SSPPCELLID0 {
                    SSPPCELLID0;
                    SSPPCELLID0 { SSPPCELLID0 }
                }
                SSPPCELLID1 {
                    SSPPCELLID1;
                    SSPPCELLID1 { SSPPCELLID1 }
                }
                SSPPCELLID2 {
                    SSPPCELLID2;
                    SSPPCELLID2 { SSPPCELLID2 }
                }
                SSPPCELLID3 {
                    SSPPCELLID3;
                    SSPPCELLID3 { SSPPCELLID3 }
                }
            }
        }
    };
}

map_spi! {
    "Extracts SPI0 register tokens.",
    periph_spi0,
    "spi0 peripheral variant.",
    Spi0,
    SPI0,
}

map_spi! {
    "Extracts SPI1 register tokens.",
    periph_spi1,
    "spi1 peripheral variant.",
    Spi1,
    SPI1,
}
