//! The Memory-Mapped Registers module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_cortexm` documentation](mod@drone_cortexm::reg).
//!
//! # API
//!
//! These API tables extend [the `drone_cortexm` API
//! tables](mod@drone_cortexm::reg).
//!
//! ## Field Token
#![doc = "

|                                                             | Field Width | Field Mode | Register Mode |
|----------------------------------------------------------------------------|-----------|-------|-------|
| [`set_bit_alias`](field::WWRegFieldBitAtomicAlias::set_bit_alias)          | one-bit   | write | write |
| [`clear_bit_alias`](field::WWRegFieldBitAtomicAlias::clear_bit_alias)      | one-bit   | write | write |
| [`toggle_bit_alias`](field::WWRegFieldBitAtomicAlias::toggle_bit_alias)    | one-bit   | write | write |
| [`set_bits_alias`](field::WWRegFieldBitsAtomicAlias::set_bits_alias)       | multi-bit | write | write |
| [`clear_bits_alias`](field::WWRegFieldBitsAtomicAlias::clear_bits_alias)   | multi-bit | write | write |
| [`toggle_bits_alias`](field::WWRegFieldBitsAtomicAlias::toggle_bits_alias) | multi-bit | write | write |

"]
//! ## Register Token
#![doc = "

|                                                                 | Mode  |
|-----------------------------------------------------------------|-------|
| [`modify_xor`](RegAtomicAliasXor::modify_xor)                   | write |
| [`modify_xor_reg`](RegAtomicAliasXor::modify_xor_reg)           | write |
| [`modify_xor_val`](RegAtomicAliasXor::modify_xor_val)           | write |
| [`modify_xor_bits`](RegAtomicAliasXor::modify_xor_bits)         | write |
| [`as_xor_alias_ptr`](RegAtomicAliasXor::as_xor_alias_ptr)       | write |
| [`modify_set`](RegAtomicAliasSet::modify_set)                   | write |
| [`modify_set_reg`](RegAtomicAliasSet::modify_set_reg)           | write |
| [`modify_set_val`](RegAtomicAliasSet::modify_set_val)           | write |
| [`modify_set_bits`](RegAtomicAliasSet::modify_set_bits)         | write |
| [`as_set_alias_ptr`](RegAtomicAliasSet::as_set_alias_ptr)       | write |
| [`modify_clear`](RegAtomicAliasClear::modify_clear)             | write |
| [`modify_clear_reg`](RegAtomicAliasClear::modify_clear_reg)     | write |
| [`modify_clear_val`](RegAtomicAliasClear::modify_clear_val)     | write |
| [`modify_clear_bits`](RegAtomicAliasClear::modify_clear_bits)   | write |
| [`as_clear_alias_ptr`](RegAtomicAliasClear::as_clear_alias_ptr) | write |

"]

pub mod field;
pub mod marker;
pub mod prelude;

use core::ptr::write_volatile;
use drone_core::bitfield::{Bitfield, Bits};
use drone_core::reg::tag::RegTag;
#[doc(no_inline)]
pub use drone_cortexm::reg::*;
pub use drone_raspberrypi_pico_map_pieces_traits::*;
#[cfg(not(feature = "std"))]
use drone_raspberrypi_pico_sdk::{REG_ALIAS_CLR_BITS, REG_ALIAS_SET_BITS, REG_ALIAS_XOR_BITS};

/// Atomic XOR register access alias.
#[allow(clippy::module_name_repetitions)]
pub trait RegAtomicAliasXor<T: RegTag>: RegAtomicAlias<T> + WReg<T> {
    /// Passes zeroed value to the closure `f`, then writes the result of the
    /// closure into the register's XOR alias memory.
    ///
    /// See also [`modify_xor_reg`](RegAtomicAliasXor::modify_xor_reg),
    /// [`modify_xor_val`](RegAtomicAliasXor::modify_xor_val),
    /// [`modify_xor_bits`](RegAtomicAliasXor::modify_xor_bits).
    fn modify_xor<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>;

    /// Passes a reference to this register token and zeroed value to the
    /// closure `f`, then writes the modified value into the register's XOR
    /// alias memory.
    ///
    /// See also [`modify_xor`](RegAtomicAliasXor::modify_xor),
    /// [`modify_xor_val`](RegAtomicAliasXor::modify_xor_val),
    /// [`modify_xor_bits`](RegAtomicAliasXor::modify_xor_bits).
    fn modify_xor_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val);

    /// Writes an opaque value `val` into the register's XOR alias memory.
    ///
    /// See also [`modify_xor`](RegAtomicAliasXor::modify_xor),
    /// [`modify_xor_bits`](RegAtomicAliasXor::modify_xor_bits).
    fn modify_xor_val(&self, val: Self::Val);

    /// Writes raw `bits` into the register's XOR alias memory.
    ///
    /// See also [`modify_xor`](RegAtomicAliasXor::modify_xor),
    /// [`modify_xor_val`](RegAtomicAliasXor::modify_xor_val).
    fn modify_xor_bits(&self, bits: <Self::Val as Bitfield>::Bits);

    /// Returns a mutable raw pointer to the register's XOR alias memory.
    fn as_xor_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits;
}

/// Atomic SET register access alias.
#[allow(clippy::module_name_repetitions)]
pub trait RegAtomicAliasSet<T: RegTag>: RegAtomicAlias<T> + WReg<T> {
    /// Passes zeroed value to the closure `f`, then writes the result of the
    /// closure into the register's SET alias memory.
    ///
    /// See also [`modify_set_reg`](RegAtomicAliasSet::modify_set_reg),
    /// [`modify_set_val`](RegAtomicAliasSet::modify_set_val),
    /// [`modify_set_bits`](RegAtomicAliasSet::modify_set_bits).
    fn modify_set<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>;

    /// Passes a reference to this register token and zeroed value to the
    /// closure `f`, then writes the modified value into the register's SET
    /// alias memory.
    ///
    /// See also [`modify_set`](RegAtomicAliasSet::modify_set),
    /// [`modify_set_val`](RegAtomicAliasSet::modify_set_val),
    /// [`modify_set_bits`](RegAtomicAliasSet::modify_set_bits).
    fn modify_set_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val);

    /// Writes an opaque value `val` into the register's SET alias memory.
    ///
    /// See also [`modify_set`](RegAtomicAliasSet::modify_set),
    /// [`modify_set_bits`](RegAtomicAliasSet::modify_set_bits).
    fn modify_set_val(&self, val: Self::Val);

    /// Writes raw `bits` into the register's SET alias memory.
    ///
    /// See also [`modify_set`](RegAtomicAliasSet::modify_set),
    /// [`modify_set_val`](RegAtomicAliasSet::modify_set_val).
    fn modify_set_bits(&self, bits: <Self::Val as Bitfield>::Bits);

    /// Returns a mutable raw pointer to the register's SET alias memory.
    fn as_set_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits;
}

/// Atomic CLEAR register access alias.
#[allow(clippy::module_name_repetitions)]
pub trait RegAtomicAliasClear<T: RegTag>: RegAtomicAlias<T> + WReg<T> {
    /// Passes zeroed value to the closure `f`, then writes the result of the
    /// closure into the register's CLEAR alias memory.
    ///
    /// See also [`modify_clear_reg`](RegAtomicAliasClear::modify_clear_reg),
    /// [`modify_clear_val`](RegAtomicAliasClear::modify_clear_val),
    /// [`modify_clear_bits`](RegAtomicAliasClear::modify_clear_bits).
    fn modify_clear<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>;

    /// Passes a reference to this register token and zeroed value to the
    /// closure `f`, then writes the modified value into the register's CLEAR
    /// alias memory.
    ///
    /// See also [`modify_clear`](RegAtomicAliasClear::modify_clear),
    /// [`modify_clear_val`](RegAtomicAliasClear::modify_clear_val),
    /// [`modify_clear_bits`](RegAtomicAliasClear::modify_clear_bits).
    fn modify_clear_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val);

    /// Writes an opaque value `val` into the register's CLEAR alias memory.
    ///
    /// See also [`modify_clear`](RegAtomicAliasClear::modify_clear),
    /// [`modify_clear_bits`](RegAtomicAliasClear::modify_clear_bits).
    fn modify_clear_val(&self, val: Self::Val);

    /// Writes raw `bits` into the register's CLEAR alias memory.
    ///
    /// See also [`modify_clear`](RegAtomicAliasClear::modify_clear),
    /// [`modify_clear_val`](RegAtomicAliasClear::modify_clear_val).
    fn modify_clear_bits(&self, bits: <Self::Val as Bitfield>::Bits);

    /// Returns a mutable raw pointer to the register's CLEAR alias memory.
    fn as_clear_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits;
}

impl<T, R> RegAtomicAliasXor<T> for R
where
    T: RegTag,
    R: RegAtomicAlias<T> + WReg<T>,
{
    #[inline]
    fn modify_xor<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>,
    {
        self.modify_xor_val(f(&mut self.hold(zero_val::<T, R>())).val());
    }

    #[inline]
    fn modify_xor_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val),
    {
        let mut val = zero_val::<T, R>();
        f(self, &mut val);
        self.modify_xor_val(val);
    }

    #[inline]
    fn modify_xor_val(&self, val: Self::Val) {
        self.modify_xor_bits(val.bits());
    }

    #[inline]
    fn modify_xor_bits(&self, bits: <Self::Val as Bitfield>::Bits) {
        unsafe { write_volatile(self.as_xor_alias_ptr(), bits) };
    }

    #[inline]
    fn as_xor_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits {
        xor_alias_ptr::<T, R>()
    }
}

impl<T, R> RegAtomicAliasSet<T> for R
where
    T: RegTag,
    R: RegAtomicAlias<T> + WReg<T>,
{
    #[inline]
    fn modify_set<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>,
    {
        self.modify_set_val(f(&mut self.hold(zero_val::<T, R>())).val());
    }

    #[inline]
    fn modify_set_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val),
    {
        let mut val = zero_val::<T, R>();
        f(self, &mut val);
        self.modify_set_val(val);
    }

    #[inline]
    fn modify_set_val(&self, val: Self::Val) {
        self.modify_set_bits(val.bits());
    }

    #[inline]
    fn modify_set_bits(&self, bits: <Self::Val as Bitfield>::Bits) {
        unsafe { write_volatile(self.as_set_alias_ptr(), bits) };
    }

    #[inline]
    fn as_set_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits {
        set_alias_ptr::<T, R>()
    }
}

impl<T, R> RegAtomicAliasClear<T> for R
where
    T: RegTag,
    R: RegAtomicAlias<T> + WReg<T>,
{
    #[inline]
    fn modify_clear<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b mut <Self as Reg<T>>::Hold<'a>) -> &'b mut <Self as Reg<T>>::Hold<'a>,
    {
        self.modify_clear_val(f(&mut self.hold(zero_val::<T, R>())).val());
    }

    #[inline]
    fn modify_clear_reg<'a, F>(&'a self, f: F)
    where
        F: for<'b> FnOnce(&'b Self, &'b mut Self::Val),
    {
        let mut val = zero_val::<T, R>();
        f(self, &mut val);
        self.modify_clear_val(val);
    }

    #[inline]
    fn modify_clear_val(&self, val: Self::Val) {
        self.modify_clear_bits(val.bits());
    }

    #[inline]
    fn modify_clear_bits(&self, bits: <Self::Val as Bitfield>::Bits) {
        unsafe { write_volatile(self.as_clear_alias_ptr(), bits) };
    }

    #[inline]
    fn as_clear_alias_ptr(&self) -> *mut <Self::Val as Bitfield>::Bits {
        clear_alias_ptr::<T, R>()
    }
}

fn xor_alias_ptr<T: RegTag, R: Reg<T>>() -> *mut <<R as Reg<T>>::Val as Bitfield>::Bits {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        (R::ADDRESS + REG_ALIAS_XOR_BITS as usize) as *mut <R::Val as Bitfield>::Bits
    }
}

fn set_alias_ptr<T: RegTag, R: Reg<T>>() -> *mut <<R as Reg<T>>::Val as Bitfield>::Bits {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        (R::ADDRESS + REG_ALIAS_SET_BITS as usize) as *mut <R::Val as Bitfield>::Bits
    }
}

fn clear_alias_ptr<T: RegTag, R: Reg<T>>() -> *mut <<R as Reg<T>>::Val as Bitfield>::Bits {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        (R::ADDRESS + REG_ALIAS_CLR_BITS as usize) as *mut <R::Val as Bitfield>::Bits
    }
}

fn zero_val<T: RegTag, R: Reg<T>>() -> <R as Reg<T>>::Val {
    unsafe { R::val_from(<<R::Val as Bitfield>::Bits as Bits>::from_usize(0)) }
}
