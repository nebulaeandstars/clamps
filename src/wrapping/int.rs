use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

use crate::macros::*;

macro_rules! impl_create {
    ($type:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> $type {
            pub fn new(inner: $inner) -> Self { Self::from(inner) }
            pub fn inner(&self) -> $inner { self.0 }
        }

        impl<const MIN: $inner, const MAX: $inner> From<$inner> for $type {
            fn from(mut inner: $inner) -> Self {
                assert!(MIN < MAX, "MIN must be less than MAX");

                if inner >= MAX {
                    let rem = (inner - MIN) % (MAX - MIN);
                    inner = MIN + rem;
                } else if inner < MIN {
                    let rem = (inner + MIN) % (MAX - MIN);
                    inner = MIN + rem;
                }

                Self(inner)
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


        // Sub takes a bit more work, as we have to factor in underflows for
        // unsigned integers in advance.
        impl_arith!($type, $other, $inner, Sub, sub, |this, mut other| {
            if other > this {
                let rem = (other + MIN) % (MAX - MIN);
                other = MIN + rem;
            }
            this - other
        });

        impl_arith_assign!($type, $other, $inner, AddAssign, add_assign, add);
        impl_arith_assign!($type, $other, $inner, MulAssign, mul_assign, mul);
        impl_arith_assign!($type, $other, $inner, DivAssign, div_assign, div);
        impl_arith_assign!($type, $other, $inner, RemAssign, rem_assign, rem);
        impl_arith_assign!($type, $other, $inner, SubAssign, sub_assign, sub);

        impl_ord!($type, $other, $inner);
    };
}

#[derive(Debug, Clone, Copy)]
pub struct WrappingU8<const MIN: u8, const MAX: u8>(u8);
impl_all!(WrappingU8<MIN, MAX>, WrappingU8<OTHER_MIN, OTHER_MAX>, u8);

#[derive(Debug, Clone, Copy)]
pub struct WrappingU16<const MIN: u16, const MAX: u16>(u16);
impl_all!(WrappingU16<MIN, MAX>, WrappingU16<OTHER_MIN, OTHER_MAX>, u16);

#[derive(Debug, Clone, Copy)]
pub struct WrappingU32<const MIN: u32, const MAX: u32>(u32);
impl_all!(WrappingU32<MIN, MAX>, WrappingU32<OTHER_MIN, OTHER_MAX>, u32);

#[derive(Debug, Clone, Copy)]
pub struct WrappingU64<const MIN: u64, const MAX: u64>(u64);
impl_all!(WrappingU64<MIN, MAX>, WrappingU64<OTHER_MIN, OTHER_MAX>, u64);

#[derive(Debug, Clone, Copy)]
pub struct WrappingU128<const MIN: u128, const MAX: u128>(u128);
impl_all!(WrappingU128<MIN, MAX>, WrappingU128<OTHER_MIN, OTHER_MAX>, u128);

#[derive(Debug, Clone, Copy)]
pub struct WrappingUSize<const MIN: usize, const MAX: usize>(usize);
impl_all!(WrappingUSize<MIN, MAX>, WrappingUSize<OTHER_MIN, OTHER_MAX>, usize);

#[derive(Debug, Clone, Copy)]
pub struct WrappingI8<const MIN: i8, const MAX: i8>(i8);
impl_all!(WrappingI8<MIN, MAX>, WrappingI8<OTHER_MIN, OTHER_MAX>, i8);

#[derive(Debug, Clone, Copy)]
pub struct WrappingI16<const MIN: i16, const MAX: i16>(i16);
impl_all!(WrappingI16<MIN, MAX>, WrappingI16<OTHER_MIN, OTHER_MAX>, i16);

#[derive(Debug, Clone, Copy)]
pub struct WrappingI32<const MIN: i32, const MAX: i32>(i32);
impl_all!(WrappingI32<MIN, MAX>, WrappingI32<OTHER_MIN, OTHER_MAX>, i32);

#[derive(Debug, Clone, Copy)]
pub struct WrappingI64<const MIN: i64, const MAX: i64>(i64);
impl_all!(WrappingI64<MIN, MAX>, WrappingI64<OTHER_MIN, OTHER_MAX>, i64);

#[derive(Debug, Clone, Copy)]
pub struct WrappingI128<const MIN: i128, const MAX: i128>(i128);
impl_all!(WrappingI128<MIN, MAX>, WrappingI128<OTHER_MIN, OTHER_MAX>, i128);

#[derive(Debug, Clone, Copy)]
pub struct WrappingISize<const MIN: isize, const MAX: isize>(isize);
impl_all!(WrappingISize<MIN, MAX>, WrappingISize<OTHER_MIN, OTHER_MAX>, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let a = WrappingU32::<3, 9>(5);
        assert_eq!(a.inner(), 5)
    }

    #[test]
    fn can_add_int() {
        let a = WrappingU32::<3, 9>(4);
        assert_eq!(a + 2, 6);
    }

    #[test]
    fn can_add_other() {
        let a = WrappingU32::<3, 9>(3);
        let b = WrappingU32::<1, 302>(4);
        assert_eq!(a + b, 7);
        assert_eq!(b + a, 7);
    }

    #[test]
    fn can_addassign_other() {
        let mut a = WrappingU32::<3, 9>(3);
        let b = WrappingU32::<0, 10>(4);
        a += b;
        assert_eq!(a.inner(), 7);
    }

    #[test]
    fn overflow_will_wrap() {
        let mut a = WrappingU32::<0, 10>(4);
        assert_eq!(a + 8, 12);

        a += 8;
        assert_ne!(a.inner(), 12);
        assert_eq!(a.inner(), 2);

        a += 4;
        assert_eq!(a.inner(), 6);

        a += 1000001;
        assert_eq!(a.inner(), 7);
    }

    #[test]
    fn bounded_underflow_will_wrap() {
        let mut a = WrappingU32::<4, 8>(6);
        assert_eq!(a - 3, 3);

        a -= 3;
        assert_ne!(a.inner(), 3);
        assert_eq!(a.inner(), 7);

        a -= 4;
        assert_eq!(a.inner(), 7);
    }

    #[test]
    fn real_underflow_will_wrap() {
        let mut a = WrappingU32::<0, 4>(2);
        a -= 4000001;
        assert_eq!(a.inner(), 1);
    }

    #[test]
    fn addassign_matches_new() {
        let mut a = WrappingU32::<0, 10>(4);

        let b = WrappingU32::<0, 10>::from(a + 8);
        a += 8;
        assert_eq!(a, b);

        let b = WrappingU32::<0, 10>::from(a + 4);
        a += 4;
        assert_eq!(a, b);

        let b = WrappingU32::<0, 10>::from(a + 1000001);
        a += 1000001;
        assert_eq!(a, b);
    }

    #[test]
    fn ord_is_implemented() {
        let a = WrappingU32::<0, 8>(5);
        let b = WrappingU32::<5, 20>(10);
        let c = 15;

        assert!(a < b);
        assert!(a < c);
        assert!(c > a.inner());
        assert!(c > b.inner());
    }

    #[test]
    fn all_types_exist() {
        let _ = WrappingU8::<0, 10>::from(5);
        let _ = WrappingU16::<0, 10>::from(5);
        let _ = WrappingU32::<0, 10>::from(5);
        let _ = WrappingU64::<0, 10>::from(5);
        let _ = WrappingU128::<0, 10>::from(5);

        let _ = WrappingI8::<-10, 10>::from(-5);
        let _ = WrappingI16::<-10, 10>::from(-5);
        let _ = WrappingI32::<-10, 10>::from(-5);
        let _ = WrappingI64::<-10, 10>::from(-5);
        let _ = WrappingI128::<-10, 10>::from(-5);
    }

    #[test]
    fn signed_integers_subtract_normally() {
        let mut foo = WrappingI32::<-10, 10>(5);
        foo -= 7;
        assert_eq!(foo, -2);
        foo -= 10;
        assert_eq!(foo, 8);
    }

    #[test]
    fn range_is_not_inclusive() {
        let mut a = WrappingUSize::<0, 10>::from(5);
        a += 5;
        assert_ne!(a, 10);
        assert_eq!(a, 0);
    }

    #[test]
    fn range_is_not_inclusive_signed() {
        let mut a = WrappingISize::<-10, 10>::from(5);
        a += 5;
        assert_ne!(a, 10);
        assert_eq!(a, -10);
    }

    #[test]
    #[should_panic]
    fn cannot_use_equal_bounds() { let _ = WrappingUSize::<10, 10>::from(5); }

    #[test]
    #[should_panic]
    fn cannot_use_invalid_bounds() { let _ = WrappingUSize::<15, 10>::from(5); }
}
