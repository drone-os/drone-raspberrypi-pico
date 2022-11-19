//! Integer Divider.

#![allow(clippy::module_name_repetitions)]

use drone_core::periph;

periph::singular! {
    /// Extracts Integer Divider register tokens.
    pub macro periph_div;

    /// Integer Divider peripheral.
    pub struct Div;

    crate::map::reg;
    crate::map::periph::div;

    SIO {
        DIV_UDIVIDEND;
        DIV_UDIVISOR;
        DIV_SDIVIDEND;
        DIV_SDIVISOR;
        DIV_QUOTIENT;
        DIV_REMAINDER;
        DIV_CSR;
    }
}
