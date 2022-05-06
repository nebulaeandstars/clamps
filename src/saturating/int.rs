use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, RangeInclusive, Rem,
    RemAssign, Sub, SubAssign,
};

use crate::macros::*;

macro_rules! impl_create {
    ($type:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> $type {
            pub fn new(inner: $inner) -> Self { Self::from(inner) }
            pub fn inner(&self) -> $inner { self.0 }
            pub fn range(&self) -> RangeInclusive<$inner> { MIN..=MAX }
            pub fn min_bound(&self) -> $inner { MIN }
            pub fn max_bound(&self) -> $inner { MAX }
        }

        impl<const MIN: $inner, const MAX: $inner> From<$inner> for $type {
            fn from(inner: $inner) -> Self {
                assert!(MIN < MAX, "MIN must be less than MAX");

                if inner >= MAX {
                    Self(MAX)
                } else if inner < MIN {
                    Self(MIN)
                } else {
                    Self(inner)
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

        impl_arith_assign!($type, $other, $inner, AddAssign, add_assign, add);
        impl_arith_assign!($type, $other, $inner, MulAssign, mul_assign, mul);
        impl_arith_assign!($type, $other, $inner, DivAssign, div_assign, div);
        impl_arith_assign!($type, $other, $inner, RemAssign, rem_assign, rem);

        impl_ord!($type, $other, $inner);

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$inner> for $type {
            fn sub_assign(&mut self, other: $inner) {
                let result = {
                    if other > self.0 - MIN {
                        MIN
                    } else {
                        self.0 - other
                    }
                };

                *self = result.into();
            }
        }

        impl<
                const MIN: $inner,
                const MAX: $inner,
                const OTHER_MIN: $inner,
                const OTHER_MAX: $inner,
            > SubAssign<$other> for $type
        {
            fn sub_assign(&mut self, other: $other) {
                self.sub_assign(other.inner())
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU8<const MIN: u8, const MAX: u8>(u8);
impl_all!(SaturatingU8<MIN, MAX>, SaturatingU8<OTHER_MIN, OTHER_MAX>, u8);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16<const MIN: u16, const MAX: u16>(u16);
impl_all!(SaturatingU16<MIN, MAX>, SaturatingU16<OTHER_MIN, OTHER_MAX>, u16);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU32<const MIN: u32, const MAX: u32>(u32);
impl_all!(SaturatingU32<MIN, MAX>, SaturatingU32<OTHER_MIN, OTHER_MAX>, u32);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU64<const MIN: u64, const MAX: u64>(u64);
impl_all!(SaturatingU64<MIN, MAX>, SaturatingU64<OTHER_MIN, OTHER_MAX>, u64);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU128<const MIN: u128, const MAX: u128>(u128);
impl_all!(SaturatingU128<MIN, MAX>, SaturatingU128<OTHER_MIN, OTHER_MAX>, u128);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingUSize<const MIN: usize, const MAX: usize>(usize);
impl_all!(SaturatingUSize<MIN, MAX>, SaturatingUSize<OTHER_MIN, OTHER_MAX>, usize);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingI8<const MIN: i8, const MAX: i8>(i8);
impl_all!(SaturatingI8<MIN, MAX>, SaturatingI8<OTHER_MIN, OTHER_MAX>, i8);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingI16<const MIN: i16, const MAX: i16>(i16);
impl_all!(SaturatingI16<MIN, MAX>, SaturatingI16<OTHER_MIN, OTHER_MAX>, i16);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingI32<const MIN: i32, const MAX: i32>(i32);
impl_all!(SaturatingI32<MIN, MAX>, SaturatingI32<OTHER_MIN, OTHER_MAX>, i32);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingI64<const MIN: i64, const MAX: i64>(i64);
impl_all!(SaturatingI64<MIN, MAX>, SaturatingI64<OTHER_MIN, OTHER_MAX>, i64);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingI128<const MIN: i128, const MAX: i128>(i128);
impl_all!(SaturatingI128<MIN, MAX>, SaturatingI128<OTHER_MIN, OTHER_MAX>, i128);

#[derive(Debug, Clone, Copy)]
pub struct SaturatingISize<const MIN: isize, const MAX: isize>(isize);
impl_all!(SaturatingISize<MIN, MAX>, SaturatingISize<OTHER_MIN, OTHER_MAX>, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let a = SaturatingU32::<3, 9>(5);
        assert_eq!(a.inner(), 5)
    }

    #[test]
    fn can_add_int() {
        let a = SaturatingU32::<3, 9>(4);
        assert_eq!(a + 2, 6);
    }

    #[test]
    fn can_add_other() {
        let a = SaturatingU32::<3, 9>(3);
        let b = SaturatingU32::<1, 302>(4);
        assert_eq!(a + b, 7);
        assert_eq!(b + a, 7);
    }

    #[test]
    fn can_addassign_other() {
        let mut a = SaturatingU32::<3, 9>(3);
        let b = SaturatingU32::<0, 10>(4);
        a += b;
        assert_eq!(a.inner(), 7);
    }

    #[test]
    fn overflow_will_saturate() {
        let mut a = SaturatingU32::<0, 10>(4);
        assert_eq!(a + 8, 12);

        a += 8;
        assert_ne!(a.inner(), 12);
        assert_eq!(a.inner(), 10);

        a += 4;
        assert_eq!(a.inner(), 10);
    }

    #[test]
    fn bounded_underflow_will_saturate() {
        let mut a = SaturatingU32::<4, 8>(6);
        assert_eq!(a - 3, 3);

        a -= 3;
        assert_ne!(a.inner(), 3);
        assert_eq!(a.inner(), 4);

        a -= 4;
        assert_eq!(a.inner(), 4);
    }

    #[test]
    fn real_underflow_will_saturate() {
        let mut a = SaturatingU32::<1, 4>(2);
        a -= 4000001;
        assert_eq!(a.inner(), 1);
    }

    #[test]
    fn addassign_matches_new() {
        let mut a = SaturatingU32::<0, 10>(4);

        let b = SaturatingU32::<0, 10>::from(a + 8);
        a += 8;
        assert_eq!(a, b);

        let b = SaturatingU32::<0, 10>::from(a + 4);
        a += 4;
        assert_eq!(a, b);

        let b = SaturatingU32::<0, 10>::from(a + 1000001);
        a += 1000001;
        assert_eq!(a, b);
    }

    #[test]
    fn ord_is_implemented() {
        let a = SaturatingU32::<0, 8>(5);
        let b = SaturatingU32::<5, 20>(10);
        let c = 15;

        assert!(a < b);
        assert!(a < c);
        assert!(c > a.inner());
        assert!(c > b.inner());
    }

    #[test]
    fn all_types_exist() {
        let _ = SaturatingU8::<0, 10>::from(5);
        let _ = SaturatingU16::<0, 10>::from(5);
        let _ = SaturatingU32::<0, 10>::from(5);
        let _ = SaturatingU64::<0, 10>::from(5);
        let _ = SaturatingU128::<0, 10>::from(5);

        let _ = SaturatingI8::<-10, 10>::from(-5);
        let _ = SaturatingI16::<-10, 10>::from(-5);
        let _ = SaturatingI32::<-10, 10>::from(-5);
        let _ = SaturatingI64::<-10, 10>::from(-5);
        let _ = SaturatingI128::<-10, 10>::from(-5);
    }

    #[test]
    fn test_arith() {
        let foo = SaturatingISize::<-100, 100>::new(5);
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
    fn test_max_and_min() {
        let foo = SaturatingISize::<-3, 8>::from(4);
        assert_eq!(foo.min_bound(), -3);
        assert_eq!(foo.max_bound(), 8);
        assert_eq!(foo.range(), -3..=8);
    }

    #[test]
    fn range_is_inclusive() {
        let mut a = SaturatingUSize::<0, 10>::from(5);
        a += 10;
        assert_eq!(a, 10);
    }

    #[test]
    fn range_is_inclusive_signed() {
        let mut a = SaturatingISize::<-10, 10>::from(5);
        a -= 20;
        assert_eq!(a, -10);
    }

    #[test]
    #[should_panic]
    fn cannot_use_equal_bounds() { let _ = SaturatingUSize::<10, 10>::from(5); }

    #[test]
    #[should_panic]
    fn cannot_use_invalid_bounds() {
        let _ = SaturatingUSize::<15, 10>::from(5);
    }
}
