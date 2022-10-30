#![warn(clippy::pedantic)]
#![no_std]

use drone_core::reg::tag::RegTag;
use drone_core::reg::Reg;

/// Atomic register access using 4 aliased memory regions.
pub trait RegAtomicAlias<T: RegTag>: Reg<T> {}
