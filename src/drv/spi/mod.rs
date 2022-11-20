//! Serial Peripheral Interface (SPI) driver.

use self::sealed::ByteOrHalfWord;
use crate::map::periph;
use crate::map::periph::spi::traits::*;
use crate::reg::prelude::*;
use core::ops::RangeInclusive;

/// Supported data frame sizes.
pub const DATA_BITS_RANGE: RangeInclusive<u8> = 4..=16;

/// Serial Peripheral Interface (SPI) driver.
pub struct Spi<T: periph::SpiMap> {
    periph: periph::Spi<T>,
}

/// SPI mode.
pub enum Mode {
    /// Master mode.
    Master,
    /// Slave mode.
    Slave {
        /// Whether to enable the output in slave mode.
        output: bool,
    },
}

/// Frame format.
pub enum FrameFormat {
    /// Motorola SPI frame format.
    Motorola {
        /// Polarity.
        spo: bool,
        /// Phase.
        sph: bool,
    },
    /// TI synchronous serial frame format.
    Ti,
    /// National Microwire frame format.
    NationalMicrowire,
}

mod sealed {
    pub trait ByteOrHalfWord: Into<u32> + 'static {
        fn from_u32(word: u32) -> Self;
    }

    impl ByteOrHalfWord for u8 {
        #[allow(clippy::cast_possible_truncation)]
        fn from_u32(word: u32) -> Self {
            word as u8
        }
    }

    impl ByteOrHalfWord for u16 {
        #[allow(clippy::cast_possible_truncation)]
        fn from_u32(word: u32) -> Self {
            word as u16
        }
    }
}

impl<T: periph::SpiMap> Spi<T> {
    /// Creates a new SPI driver from the peripheral.
    #[inline]
    #[must_use]
    pub fn new(periph: periph::Spi<T>) -> Self {
        Self { periph }
    }

    /// Releases the SPI peripheral.
    #[inline]
    #[must_use]
    pub fn free(self) -> periph::Spi<T> {
        self.periph
    }

    /// Configures the SPI.
    ///
    /// Resulting frequency is `clk_peri / (prescale * (1 + postdiv))`.
    ///
    /// SPI must be disabled when calling this method.
    ///
    /// # Panics
    ///
    /// `prescale` must be an even number from 2-254.
    pub fn init(
        &self,
        prescale: u8,
        postdiv: u8,
        data_bits: u8,
        frame_format: &FrameFormat,
        mode: &Mode,
    ) {
        assert!(prescale % 2 == 0, "prescalse must be even");
        assert!(prescale > 0, "prescale can't be 0");
        assert!(DATA_BITS_RANGE.contains(&data_bits), "data_bits must be 4-16");
        self.periph.spi_sspcpsr.store_reg(|r, v| {
            r.cpsdvsr().write(v, u32::from(prescale));
        });
        self.periph.spi_sspcr0.store_reg(|r, v| {
            r.scr().write(v, u32::from(postdiv));
            r.frf().write(v, match frame_format {
                FrameFormat::Motorola { .. } => 0b00,
                FrameFormat::Ti => 0b01,
                FrameFormat::NationalMicrowire => 0b10,
            });
            if let FrameFormat::Motorola { spo, sph } = *frame_format {
                r.spo().write(v, spo);
                r.sph().write(v, sph);
            }
            r.dss().write(v, u32::from(data_bits - 1));
        });
        self.periph.spi_sspcr1.store_reg(|r, v| match mode {
            Mode::Master => {
                r.ms().clear(v);
            }
            Mode::Slave { output } => {
                r.ms().set(v);
                r.sod().write(v, !output);
            }
        });
    }

    /// Enables DMA DREQ signals.
    #[inline]
    pub fn enable_dma(&self, tx: bool, rx: bool) {
        self.periph.spi_sspdmacr.store_reg(|r, v| {
            if tx {
                r.txdmae().set(v);
            }
            if rx {
                r.rxdmae().set(v);
            }
        });
    }

    /// Enables SPI operation.
    #[inline]
    pub fn enable(&self) {
        self.periph.spi_sspcr1.modify_set_reg(|r, v| {
            r.sse().set(v);
        });
    }

    /// Disables SPI operation.
    #[inline]
    pub fn disable(&self) {
        self.periph.spi_sspcr1.modify_clear_reg(|r, v| {
            r.sse().set(v);
        });
    }

    /// Write bytes from `src` to SPI. Simultaneously read bytes from SPI to
    /// `dst`. Transfer size is determined by `src` length. Blocks until all
    /// data is transferred. No timeout, as SPI hardware always transfers at
    /// a known data rate.
    ///
    /// # Examples
    ///
    /// Write 4 bytes and read 4 bytes:
    ///
    /// ```no_run
    /// # use drone_raspberrypi_pico::map::rp2040_reg_tokens;
    /// # rp2040_reg_tokens! { index => pub Regs; }
    /// # let reg = unsafe { <Regs as drone_core::token::Token>::take() };
    /// use drone_raspberrypi_pico::drv::Spi;
    /// use drone_raspberrypi_pico::periph_spi0;
    ///
    /// let spi = Spi::new(periph_spi0!(reg));
    /// let mut response = [0_u8; 4];
    /// spi.write_read_blocking([0, 1, 2, 3], &mut response);
    /// ```
    ///
    /// Write 4 bytes and discard any data received back:
    ///
    /// ```no_run
    /// # use drone_raspberrypi_pico::map::rp2040_reg_tokens;
    /// # rp2040_reg_tokens! { index => pub Regs; }
    /// # let reg = unsafe { <Regs as drone_core::token::Token>::take() };
    /// use drone_raspberrypi_pico::drv::Spi;
    /// use drone_raspberrypi_pico::periph_spi0;
    ///
    /// let spi = Spi::new(periph_spi0!(reg));
    /// spi.write_read_blocking([0_u8, 1, 2, 3], []);
    /// ```
    ///
    /// Write 4 repeated `0xFF` bytes and read 4 bytes:
    ///
    /// ```no_run
    /// # use drone_raspberrypi_pico::map::rp2040_reg_tokens;
    /// # rp2040_reg_tokens! { index => pub Regs; }
    /// # let reg = unsafe { <Regs as drone_core::token::Token>::take() };
    /// use core::iter;
    /// use drone_raspberrypi_pico::drv::Spi;
    /// use drone_raspberrypi_pico::periph_spi0;
    ///
    /// let spi = Spi::new(periph_spi0!(reg));
    /// let mut response = [0_u8; 4];
    /// spi.write_read_blocking(iter::repeat(0xFF).take(4), &mut response);
    /// ```
    pub fn write_read_blocking<'d, V, S, D>(&self, src: S, dst: D)
    where
        S: IntoIterator<Item = V>,
        D: IntoIterator<Item = &'d mut V>,
        V: ByteOrHalfWord,
    {
        let mut src = src.into_iter();
        let mut dst = dst.into_iter().fuse();
        let mut src_sent = 0;
        let mut dst_recv = 0;
        let mut src_exhausted = false;
        while !src_exhausted || src_sent != dst_recv {
            let sr = self.periph.spi_sspsr.load_val();
            if !src_exhausted && self.periph.spi_sspsr.tnf().read(&sr) {
                if let Some(src) = src.next() {
                    self.periph.spi_sspdr.store_bits(src.into());
                    src_sent += 1;
                } else {
                    src_exhausted = true;
                }
            }
            if self.periph.spi_sspsr.rne().read(&sr) {
                let data = self.periph.spi_sspdr.load_bits();
                if let Some(dst) = dst.next() {
                    *dst = V::from_u32(data);
                    dst_recv += 1;
                }
            }
        }
    }
}
