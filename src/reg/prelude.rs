//! The Memory-Mapped Registers prelude.
//!
//! The purpose of this module is to alleviate imports of many common `reg`
//! traits by adding a glob import to the top of `reg` heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_raspberrypi_pico::reg::prelude::*;
//! ```

pub use crate::reg::field::{WWRegFieldBitAtomicAlias, WWRegFieldBitsAtomicAlias};
pub use crate::reg::{RegAtomicAlias, RegAtomicAliasClear, RegAtomicAliasSet, RegAtomicAliasXor};
#[doc(no_inline)]
pub use drone_cortexm::reg::prelude::*;
