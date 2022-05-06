use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Range, Rem, Sub};

use super::BoundsError;
use crate::macros::*;

macro_rules! impl_create {
    ($type:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> $type {
            pub fn new(inner: $inner) -> Result<Self, BoundsError> {
                Self::try_from(inner)
            }
            pub fn inner(&self) -> $inner { self.0 }
            pub fn range(&self) -> Range<$inner> { MIN..MAX }
            pub fn min_bound(&self) -> $inner { MIN }
            pub fn max_bound(&self) -> $inner { MAX }
        }

        impl<const MIN: $inner, const MAX: $inner> TryFrom<$inner> for $type {
            type Error = BoundsError;
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                assert!(MIN < MAX, "MIN must be less than MAX");

                if inner >= MAX {
                    Err(BoundsError::TooLarge)
                } else if inner < MIN {
                    Err(BoundsError::TooSmall)
                } else {
                    Ok(Self(inner))
                }
            }
        }
    };
}

macro_rules! impl_all {
    ($type:ty, $other:ty, $inner:ty) => {
        impl_create!($type, $inner);

        impl_arith!($type, $other, $inner, Add, add, |this, other| this
            + other);
        impl_arith!($type, $other, $inner, Div, div, |this, other| this
            / other);
        impl_arith!($type, $other, $inner, Mul, mul, |this, other| this
            * other);
        impl_arith!($type, $other, $inner, Rem, rem, |this, other| this
            % other);
        impl_arith!($type, $other, $inner, Sub, sub, |this, other| this
            - other);

        impl_ord!($type, $other, $inner);
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BoundedU8<const MIN: u8, const MAX: u8>(u8);
impl_all!(BoundedU8<MIN, MAX>, BoundedU8<OTHER_MIN, OTHER_MAX>, u8);

#[derive(Debug, Clone, Copy)]
pub struct BoundedU16<const MIN: u16, const MAX: u16>(u16);
impl_all!(BoundedU16<MIN, MAX>, BoundedU16<OTHER_MIN, OTHER_MAX>, u16);

#[derive(Debug, Clone, Copy)]
pub struct BoundedU32<const MIN: u32, const MAX: u32>(u32);
impl_all!(BoundedU32<MIN, MAX>, BoundedU32<OTHER_MIN, OTHER_MAX>, u32);

#[derive(Debug, Clone, Copy)]
pub struct BoundedU64<const MIN: u64, const MAX: u64>(u64);
impl_all!(BoundedU64<MIN, MAX>, BoundedU64<OTHER_MIN, OTHER_MAX>, u64);

#[derive(Debug, Clone, Copy)]
pub struct BoundedU128<const MIN: u128, const MAX: u128>(u128);
impl_all!(BoundedU128<MIN, MAX>, BoundedU128<OTHER_MIN, OTHER_MAX>, u128);

#[derive(Debug, Clone, Copy)]
pub struct BoundedUSize<const MIN: usize, const MAX: usize>(usize);
impl_all!(BoundedUSize<MIN, MAX>, BoundedUSize<OTHER_MIN, OTHER_MAX>, usize);

#[derive(Debug, Clone, Copy)]
pub struct BoundedI8<const MIN: i8, const MAX: i8>(i8);
impl_all!(BoundedI8<MIN, MAX>, BoundedI8<OTHER_MIN, OTHER_MAX>, i8);

#[derive(Debug, Clone, Copy)]
pub struct BoundedI16<const MIN: i16, const MAX: i16>(i16);
impl_all!(BoundedI16<MIN, MAX>, BoundedI16<OTHER_MIN, OTHER_MAX>, i16);

#[derive(Debug, Clone, Copy)]
pub struct BoundedI32<const MIN: i32, const MAX: i32>(i32);
impl_all!(BoundedI32<MIN, MAX>, BoundedI32<OTHER_MIN, OTHER_MAX>, i32);

#[derive(Debug, Clone, Copy)]
pub struct BoundedI64<const MIN: i64, const MAX: i64>(i64);
impl_all!(BoundedI64<MIN, MAX>, BoundedI64<OTHER_MIN, OTHER_MAX>, i64);

#[derive(Debug, Clone, Copy)]
pub struct BoundedI128<const MIN: i128, const MAX: i128>(i128);
impl_all!(BoundedI128<MIN, MAX>, BoundedI128<OTHER_MIN, OTHER_MAX>, i128);

#[derive(Debug, Clone, Copy)]
pub struct BoundedISize<const MIN: isize, const MAX: isize>(isize);
impl_all!(BoundedISize<MIN, MAX>, BoundedISize<OTHER_MIN, OTHER_MAX>, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let a = BoundedU32::<3, 9>(5);
        assert_eq!(a.inner(), 5)
    }

    #[test]
    fn can_add_int() {
        let a = BoundedU32::<3, 9>(4);
        assert_eq!(a + 2, 6);
    }

    #[test]
    fn can_add_other() {
        let a = BoundedU32::<3, 9>(3);
        let b = BoundedU32::<1, 302>(4);
        assert_eq!(a + b, 7);
        assert_eq!(b + a, 7);
    }

    #[test]
    fn cannot_create_outside_of_bounds() {
        use BoundsError::*;
        assert_eq!(BoundedU32::<2, 10>::try_from(4).unwrap(), 4);
        assert_eq!(BoundedU32::<2, 10>::try_from(1), Err(TooSmall));
        assert_eq!(BoundedU32::<2, 10>::try_from(14), Err(TooLarge));
    }

    #[test]
    fn ord_is_implemented() {
        let a = BoundedU32::<0, 8>(5);
        let b = BoundedU32::<5, 20>(10);
        let c = 15;

        assert!(a < b);
        assert!(a < c);
        assert!(c > a.inner());
        assert!(c > b.inner());
    }

    #[test]
    fn test_max_and_min() {
        let foo = BoundedISize::<-3, 8>::try_from(4).unwrap();
        assert_eq!(foo.min_bound(), -3);
        assert_eq!(foo.max_bound(), 8);
        assert_eq!(foo.range(), -3..8);
    }

    #[test]
    fn test_arith() {
        let foo = BoundedISize::<-100, 100>::new(5).unwrap();
        for num in -10..10 {
            assert_eq!(foo + num, foo.inner() + num);
            assert_eq!(foo - num, foo.inner() - num);
            assert_eq!(foo * num, foo.inner() * num);

            if num != 0 {
                assert_eq!(foo / num, foo.inner() / num);
                assert_eq!(foo % num, foo.inner() % num);
            }
        }
    }

    #[test]
    fn all_types_exist() {
        let _ = BoundedU8::<0, 10>::try_from(5).unwrap();
        let _ = BoundedU16::<0, 10>::try_from(5).unwrap();
        let _ = BoundedU32::<0, 10>::try_from(5).unwrap();
        let _ = BoundedU64::<0, 10>::try_from(5).unwrap();
        let _ = BoundedU128::<0, 10>::try_from(5).unwrap();

        let _ = BoundedI8::<-10, 10>::try_from(-5).unwrap();
        let _ = BoundedI16::<-10, 10>::try_from(-5).unwrap();
        let _ = BoundedI32::<-10, 10>::try_from(-5).unwrap();
        let _ = BoundedI64::<-10, 10>::try_from(-5).unwrap();
        let _ = BoundedI128::<-10, 10>::try_from(-5).unwrap();
    }

    #[test]
    #[should_panic]
    fn cannot_use_equal_bounds() {
        let _ = BoundedUSize::<10, 10>::try_from(5);
    }

    #[test]
    #[should_panic]
    fn cannot_use_invalid_bounds() {
        let _ = BoundedUSize::<15, 10>::try_from(5);
    }
}
