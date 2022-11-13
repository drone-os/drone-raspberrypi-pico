//! Memory-mapped register fields module.
//!
//! See [the top-level module documentation](self) for details.

use super::{clear_alias_ptr, set_alias_ptr, xor_alias_ptr};
use crate::reg::RegAtomicAlias;
use core::ptr::write_volatile;
use drone_core::bitfield::{Bitfield, Bits};
use drone_core::reg::field::WWRegFieldBit;
use drone_core::reg::tag::RegTag;
use drone_core::reg::{Reg, WReg};
#[doc(no_inline)]
pub use drone_cortexm::reg::field::*;

/// Writable single-bit field of writable register with atomic register access
/// using 4 aliased memory regions.
#[allow(clippy::upper_case_acronyms)]
pub trait WWRegFieldBitAtomicAlias<T: RegTag>
where
    Self: WWRegFieldBit<T>,
    Self::Reg: RegAtomicAlias<T> + WReg<T>,
{
    /// Writes zeroed value with this bit set to 1 into the register's SET alias
    /// memory.
    fn set_bit_alias(&self);

    /// Writes zeroed value with this bit set to 1 into the register's CLEAR
    /// alias memory.
    fn clear_bit_alias(&self);

    /// Writes zeroed value with this bit set to 1 into the register's XOR alias
    /// memory.
    fn toggle_bit_alias(&self);
}

/// Writable multiple-bit field of writable register with atomic register access
/// using 4 aliased memory regions.
#[allow(clippy::upper_case_acronyms)]
pub trait WWRegFieldBitsAtomicAlias<T: RegTag>
where
    Self: WWRegFieldBits<T>,
    Self::Reg: RegAtomicAlias<T> + WReg<T>,
{
    /// Writes zeroed value with this field bits replaced by `bits` into the
    /// register's SET alias memory.
    fn set_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits);

    /// Writes zeroed value with this field bits replaced by `bits` into the
    /// register's CLEAR alias memory.
    fn clear_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits);

    /// Writes zeroed value with this field bits replaced by `bits` into the
    /// register's XOR alias memory.
    fn toggle_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits);
}

impl<T, R> WWRegFieldBitAtomicAlias<T> for R
where
    T: RegTag,
    R: WWRegFieldBit<T>,
    R::Reg: RegAtomicAlias<T> + WReg<T>,
{
    #[inline]
    fn set_bit_alias(&self) {
        store_val::<T, R::Reg>(set_alias_ptr::<T, R::Reg>(), |val| {
            self.set(val);
        });
    }

    #[inline]
    fn clear_bit_alias(&self) {
        store_val::<T, R::Reg>(clear_alias_ptr::<T, R::Reg>(), |val| {
            self.set(val);
        });
    }

    #[inline]
    fn toggle_bit_alias(&self) {
        store_val::<T, R::Reg>(xor_alias_ptr::<T, R::Reg>(), |val| {
            self.set(val);
        });
    }
}

impl<T, R> WWRegFieldBitsAtomicAlias<T> for R
where
    T: RegTag,
    R: WWRegFieldBits<T>,
    R::Reg: RegAtomicAlias<T> + WReg<T>,
{
    #[inline]
    fn set_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits) {
        store_val::<T, R::Reg>(set_alias_ptr::<T, R::Reg>(), |val| {
            self.write(val, bits);
        });
    }

    #[inline]
    fn clear_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits) {
        store_val::<T, R::Reg>(clear_alias_ptr::<T, R::Reg>(), |val| {
            self.write(val, bits);
        });
    }

    #[inline]
    fn toggle_bits_alias(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits) {
        store_val::<T, R::Reg>(xor_alias_ptr::<T, R::Reg>(), |val| {
            self.write(val, bits);
        });
    }
}

fn store_val<T: RegTag, R: Reg<T>>(
    ptr: *mut <<R as Reg<T>>::Val as Bitfield>::Bits,
    f: impl Fn(&mut <R as Reg<T>>::Val),
) {
    let mut val = unsafe { R::val_from(<<R::Val as Bitfield>::Bits as Bits>::from_usize(0)) };
    f(&mut val);
    unsafe { write_volatile(ptr, val.bits()) };
}
