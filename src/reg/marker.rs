//! Marker traits representing properties of memory-mapped registers.

#![allow(clippy::type_repetition_in_bounds)]

use crate::reg::field::{WWRegFieldBitAtomicAlias, WWRegFieldBitsAtomicAlias};
use crate::reg::tag::{Crt, RegTag, Srt, Urt};
use crate::reg::{RegAtomicAlias, WReg};
#[doc(inline)]
pub use drone_cortexm::reg::marker::*;

/// Read-write register with atomic access using 4 aliased memory regions.
#[marker]
pub trait RwRegAtomicAlias<T: RegTag>
where
    Self: RwReg<T>,
    Self: RegAtomicAlias<T>,
{
}

impl<R, T: RegTag> RwRegAtomicAlias<T> for R
where
    R: RwReg<T>,
    R: RegAtomicAlias<T>,
{
}

/// Read-only register with atomic access using 4 aliased memory regions.
#[marker]
pub trait RoRegAtomicAlias<T: RegTag>
where
    Self: RoReg<T>,
    Self: RegAtomicAlias<T>,
{
}

impl<R, T: RegTag> RoRegAtomicAlias<T> for R
where
    R: RoReg<T>,
    R: RegAtomicAlias<T>,
{
}

/// Write-only register with atomic access using 4 aliased memory regions.
#[marker]
pub trait WoRegAtomicAlias<T: RegTag>
where
    Self: WoReg<T>,
    Self: RegAtomicAlias<T>,
{
}

impl<R, T: RegTag> WoRegAtomicAlias<T> for R
where
    R: WoReg<T>,
    R: RegAtomicAlias<T>,
{
}

/// Unsynchronized read-write register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait URwRegAtomicAlias
where
    Self: URwReg,
    Self: RegAtomicAlias<Urt>,
{
}

impl<R> URwRegAtomicAlias for R
where
    R: URwReg,
    R: RegAtomicAlias<Urt>,
{
}

/// Unsynchronized read-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait URoRegAtomicAlias
where
    Self: URoReg,
    Self: RegAtomicAlias<Urt>,
{
}

impl<R> URoRegAtomicAlias for R
where
    R: URoReg,
    R: RegAtomicAlias<Urt>,
{
}

/// Unsynchronized write-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait UWoRegAtomicAlias
where
    Self: UWoReg,
    Self: RegAtomicAlias<Urt>,
{
}

impl<R> UWoRegAtomicAlias for R
where
    R: UWoReg,
    R: RegAtomicAlias<Urt>,
{
}

/// Synchronized read-write register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait SRwRegAtomicAlias
where
    Self: SRwReg,
    Self: RegAtomicAlias<Srt>,
{
}

impl<R> SRwRegAtomicAlias for R
where
    R: SRwReg,
    R: RegAtomicAlias<Srt>,
{
}

/// Synchronized read-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait SRoRegAtomicAlias
where
    Self: SRoReg,
    Self: RegAtomicAlias<Srt>,
{
}

impl<R> SRoRegAtomicAlias for R
where
    R: SRoReg,
    R: RegAtomicAlias<Srt>,
{
}

/// Synchronized write-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait SWoRegAtomicAlias
where
    Self: SWoReg,
    Self: RegAtomicAlias<Srt>,
{
}

impl<R> SWoRegAtomicAlias for R
where
    R: SWoReg,
    R: RegAtomicAlias<Srt>,
{
}

/// Copyable read-write register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait CRwRegAtomicAlias
where
    Self: CRwReg,
    Self: RegAtomicAlias<Crt>,
{
}

impl<R> CRwRegAtomicAlias for R
where
    R: CRwReg,
    R: RegAtomicAlias<Crt>,
{
}

/// Copyable read-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait CRoRegAtomicAlias
where
    Self: CRoReg,
    Self: RegAtomicAlias<Crt>,
{
}

impl<R> CRoRegAtomicAlias for R
where
    R: CRoReg,
    R: RegAtomicAlias<Crt>,
{
}

/// Copyable write-only register with atomic access using 4 aliased memory
/// regions.
#[marker]
pub trait CWoRegAtomicAlias
where
    Self: CWoReg,
    Self: RegAtomicAlias<Crt>,
{
}

impl<R> CWoRegAtomicAlias for R
where
    R: CWoReg,
    R: RegAtomicAlias<Crt>,
{
}

/// Unsynchronized writable single-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait UWWRegFieldBitAtomicAlias
where
    Self: WWRegFieldBitAtomicAlias<Urt>,
    Self::Reg: RegAtomicAlias<Urt> + WReg<Urt>,
{
}

impl<R> UWWRegFieldBitAtomicAlias for R
where
    R: WWRegFieldBitAtomicAlias<Urt>,
    R::Reg: RegAtomicAlias<Urt> + WReg<Urt>,
{
}

/// Synchronized writable single-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait SWWRegFieldBitAtomicAlias
where
    Self: WWRegFieldBitAtomicAlias<Srt>,
    Self::Reg: RegAtomicAlias<Srt> + WReg<Srt>,
{
}

impl<R> SWWRegFieldBitAtomicAlias for R
where
    R: WWRegFieldBitAtomicAlias<Srt>,
    R::Reg: RegAtomicAlias<Srt> + WReg<Srt>,
{
}

/// Copyable writable single-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait CWWRegFieldBitAtomicAlias
where
    Self: WWRegFieldBitAtomicAlias<Crt>,
    Self: Copy,
    Self::Reg: RegAtomicAlias<Crt> + WReg<Crt>,
{
}

impl<R> CWWRegFieldBitAtomicAlias for R
where
    R: WWRegFieldBitAtomicAlias<Crt>,
    R: Copy,
    R::Reg: RegAtomicAlias<Crt> + WReg<Crt>,
{
}

/// Unsynchronized writable multiple-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait UWWRegFieldBitsAtomicAlias
where
    Self: WWRegFieldBitsAtomicAlias<Urt>,
    Self::Reg: RegAtomicAlias<Urt> + WReg<Urt>,
{
}

impl<R> UWWRegFieldBitsAtomicAlias for R
where
    R: WWRegFieldBitsAtomicAlias<Urt>,
    R::Reg: RegAtomicAlias<Urt> + WReg<Urt>,
{
}

/// Synchronized writable multiple-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait SWWRegFieldBitsAtomicAlias
where
    Self: WWRegFieldBitsAtomicAlias<Srt>,
    Self::Reg: RegAtomicAlias<Srt> + WReg<Srt>,
{
}

impl<R> SWWRegFieldBitsAtomicAlias for R
where
    R: WWRegFieldBitsAtomicAlias<Srt>,
    R::Reg: RegAtomicAlias<Srt> + WReg<Srt>,
{
}

/// Copyable writable multiple-bit field of writable register with atomic
/// register access using 4 aliased memory regions.
#[marker]
pub trait CWWRegFieldBitsAtomicAlias
where
    Self: WWRegFieldBitsAtomicAlias<Crt>,
    Self: Copy,
    Self::Reg: RegAtomicAlias<Crt> + WReg<Crt>,
{
}

impl<R> CWWRegFieldBitsAtomicAlias for R
where
    R: WWRegFieldBitsAtomicAlias<Crt>,
    R: Copy,
    R::Reg: RegAtomicAlias<Crt> + WReg<Crt>,
{
}
