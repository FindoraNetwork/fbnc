//!
//! A storage type for various kinds of single value.
//!
//! NOTE:
//! - Values will be encoded by some `serde`-like methods
//!
//! # Examples
//!
//! ```
//! use vsdb::Orphan;
//!
//! assert_eq!(Orphan::new(0), 0);
//! assert!(Orphan::new(1) > 0);
//! assert!(Orphan::new(1) >= 0);
//! assert!(Orphan::new(0) < 1);
//! assert!(Orphan::new(0) <= 1);
//!
//! assert_eq!(Orphan::new(0), Orphan::new(0));
//! assert!(Orphan::new(1) > Orphan::new(0));
//! assert!(Orphan::new(1) >= Orphan::new(1));
//! assert!(Orphan::new(0) < Orphan::new(1));
//! assert!(Orphan::new(1) <= Orphan::new(1));
//!
//! assert_eq!(Orphan::new(1) + 1, 2);
//! assert_eq!(Orphan::new(1) - 1, 0);
//! assert_eq!(Orphan::new(1) * 1, 1);
//! assert_eq!(Orphan::new(1) / 2, 0);
//! assert_eq!(Orphan::new(1) % 2, 1);
//!
//! assert_eq!(-Orphan::new(1), -1);
//! assert_eq!(!Orphan::new(1), !1);
//!
//! assert_eq!(Orphan::new(1) >> 2, 1 >> 2);
//! assert_eq!(Orphan::new(1) << 2, 1 << 2);
//!
//! assert_eq!(Orphan::new(1) | 2, 1 | 2);
//! assert_eq!(Orphan::new(1) & 2, 1 & 2);
//! assert_eq!(Orphan::new(1) ^ 2, 1 ^ 2);
//!
//! let mut v = Orphan::new(1);
//! v += 1;
//! assert_eq!(v, 2);
//! v *= 100;
//! assert_eq!(v, 200);
//! v -= 1;
//! assert_eq!(v, 199);
//! v /= 10;
//! assert_eq!(v, 19);
//! v %= 10;
//! assert_eq!(v, 9);
//!
//! *v.get_mut() = -v.clone_inner();
//! assert_eq!(v, -9);
//!
//! *v.get_mut() = !v.clone_inner();
//! assert_eq!(v, !-9);
//!
//! *v.get_mut() = 0;
//! v >>= 2;
//! assert_eq!(v, 0 >> 2);
//!
//! *v.get_mut() = 0;
//! v <<= 2;
//! assert_eq!(v, 0 << 2);
//!
//! *v.get_mut() = 0;
//! v |= 2;
//! assert_eq!(v, 0 | 2);
//!
//! *v.get_mut() = 0;
//! v &= 2;
//! assert_eq!(v, 0 & 2);
//!
//! *v.get_mut() = 0;
//! v ^= 2;
//! assert_eq!(v, 0 ^ 2);
//! ```

#[cfg(test)]
mod test;

use crate::{basic::mapx_ord_rawkey::MapxOrdRawKey, ValueEnDe};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
        Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// Used to express some 'non-collection' types,
/// such as any type of integer, an enum value, etc..
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq)]
#[serde(bound = "")]
pub struct Orphan<T>
where
    T: ValueEnDe,
{
    inner: MapxOrdRawKey<T>,
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

impl<T> Orphan<T>
where
    T: ValueEnDe,
{
    pub fn new(v: T) -> Self {
        let mut hdr = MapxOrdRawKey::new();
        hdr.insert_ref(&[], &v);
        Self { inner: hdr }
    }

    /// Clone the inner value.
    pub fn clone_inner(&self) -> T {
        self.get_value()
    }

    fn get_value(&self) -> T {
        self.inner.get(&[]).unwrap()
    }

    fn set_value_ref(&mut self, v: &T) {
        self.inner.set_value_ref(&[], v);
    }

    /// Get the mutable handler of the value.
    ///
    /// NOTE:
    /// - Always use this method to change value
    ///     - `*(<Orphan>).get_mut() = ...`
    /// - **NEVER** do this:
    ///     - `*(&mut <Orphan>) = Orphan::new(...)`
    ///     - OR you will loss the 'versioned' ability of this object
    pub fn get_mut(&mut self) -> OrphanMut<'_, T> {
        let value = self.get_value();
        OrphanMut { hdr: self, value }
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

impl<T> PartialEq for Orphan<T>
where
    T: ValueEnDe + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl<T> PartialEq<T> for Orphan<T>
where
    T: ValueEnDe + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get_value() == *other
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

impl<T> Ord for Orphan<T>
where
    T: ValueEnDe + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl<T> PartialOrd for Orphan<T>
where
    T: ValueEnDe + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl<T> PartialOrd<T> for Orphan<T>
where
    T: ValueEnDe + Ord,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.get_value().partial_cmp(other)
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

macro_rules! impl_ops {
    ($ops: tt, $fn: tt, $op: tt) => {
        impl<T> $ops for Orphan<T>
        where
            T: ValueEnDe + Ord + Eq + $ops<Output = T>,
        {
            type Output = T;
            fn $fn(self, other: Self) -> Self::Output {
                self.get_value() $op other.get_value()
            }
        }

        impl<T> $ops<T> for Orphan<T>
        where
            T: ValueEnDe + Ord + Eq + $ops<Output = T>,
        {
            type Output = T;
            fn $fn(self, other: T) -> Self::Output {
                self.get_value() $op other
            }
        }
    };
    ($ops: tt, $fn: tt, $op: tt, $ops_assign: tt, $fn_assign: tt, $op_assign: tt) => {
        impl_ops!($ops, $fn, $op);

        impl<T> $ops_assign for Orphan<T>
        where
            T: ValueEnDe + Ord + Eq + $ops_assign,
        {
            fn $fn_assign(&mut self, other: Self) {
                *self.get_mut() $op_assign other.get_value();
            }
        }

        impl<T> $ops_assign<T> for Orphan<T>
        where
            T: ValueEnDe + Ord + Eq + $ops_assign,
        {
            fn $fn_assign(&mut self, other: T) {
                *self.get_mut() $op_assign other;
            }
        }
    };
    (@$ops: tt, $fn: tt, $op: tt) => {
        impl<T> $ops for Orphan<T>
        where
            T: ValueEnDe + Ord + Eq + $ops<Output = T>,
        {
            type Output = T;
            fn $fn(self) -> Self::Output {
                $op self.get_value()
            }
        }
    };
}

impl_ops!(Add, add, +, AddAssign, add_assign, +=);
impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=);
impl_ops!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_ops!(Div, div, /, DivAssign, div_assign, /=);
impl_ops!(Rem, rem, %, RemAssign, rem_assign, %=);

impl_ops!(BitAnd, bitand, &, BitAndAssign, bitand_assign, &=);
impl_ops!(BitOr, bitor, |, BitOrAssign, bitor_assign, |=);
impl_ops!(BitXor, bitxor, ^, BitXorAssign, bitxor_assign, ^=);
impl_ops!(Shl, shl, <<, ShlAssign, shl_assign, <<=);
impl_ops!(Shr, shr, >>, ShrAssign, shr_assign, >>=);

impl_ops!(@Not, not, !);
impl_ops!(@Neg, neg, -);

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// A type returned by `get_mut()`.
pub struct OrphanMut<'a, T>
where
    T: ValueEnDe,
{
    hdr: &'a mut Orphan<T>,
    value: T,
}

impl<'a, T> Drop for OrphanMut<'a, T>
where
    T: ValueEnDe,
{
    fn drop(&mut self) {
        self.hdr.set_value_ref(&self.value);
    }
}

impl<'a, T> Deref for OrphanMut<'a, T>
where
    T: ValueEnDe,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> DerefMut for OrphanMut<'a, T>
where
    T: ValueEnDe,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

impl<T> Iterator for Orphan<T>
where
    T: ValueEnDe + Eq,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_value())
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////