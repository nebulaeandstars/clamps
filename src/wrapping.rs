use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy)]
pub struct WrappingU32<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> WrappingU32<MIN, MAX> {
    pub fn new(inner: u32) -> Self {
        Self::from(inner)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl<const MIN: u32, const MAX: u32> From<u32> for WrappingU32<MIN, MAX> {
    fn from(mut inner: u32) -> Self {
        if inner > MAX {
            let rem = (inner - MIN) % (MAX - MIN);
            inner = rem + MIN;
        }

        Self(inner)
    }
}

// provides an easier way to define two implementations:
// - impl $trait<u32> for WrappingT<MIN, MAX>
// - impl $trait<WrappingT<OTHER_MIN, OTHER_MAX>> for WrappingT<MIN, MAX>
//
// The generics were getting a bit ridiculous, so this short-hand exists to make
// defining arithmetic traits like Add, Sub, etc. much easier.
macro_rules! arith_impl {
    ($trait:ident, $fn:ident, $impl:expr) => {
        impl<const MIN: u32, const MAX: u32> $trait<u32> for WrappingU32<MIN, MAX> {
            type Output = u32;
            fn $fn(self, other: u32) -> Self::Output {
                $impl(self.0, other)
            }
        }

        impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
            $trait<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
        {
            type Output = u32;
            fn $fn(self, other: WrappingU32<OTHER_MIN, OTHER_MAX>) -> Self::Output {
                $impl(self.0, other.0)
            }
        }
    };
}

macro_rules! arith_assign_impl {
    ($trait:ident, $fn:ident, $op:ident) => {
        impl<const MIN: u32, const MAX: u32> $trait<u32> for WrappingU32<MIN, MAX> {
            fn $fn(&mut self, other: u32) {
                *self = self.$op(other).into()
            }
        }

        impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
            $trait<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
        {
            fn $fn(&mut self, other: WrappingU32<OTHER_MIN, OTHER_MAX>) {
                *self = self.$op(other).into()
            }
        }
    };
}

arith_impl!(Add, add, |this, other| this + other);
arith_impl!(Sub, sub, |this, other| this - other);
arith_impl!(Div, div, |this, other| this / other);
arith_impl!(Mul, mul, |this, other| this * other);
arith_impl!(Rem, rem, |this, other| this % other);

arith_assign_impl!(AddAssign, add_assign, add);
arith_assign_impl!(SubAssign, sub_assign, sub);
arith_assign_impl!(MulAssign, mul_assign, mul);
arith_assign_impl!(DivAssign, div_assign, div);
arith_assign_impl!(RemAssign, rem_assign, rem);

impl<const MIN: u32, const MAX: u32> PartialEq<u32> for WrappingU32<MIN, MAX> {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
    PartialEq<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
{
    fn eq(&self, other: &WrappingU32<OTHER_MIN, OTHER_MAX>) -> bool {
        self.0 == other.0
    }
}
impl<const MIN: u32, const MAX: u32> Eq for WrappingU32<MIN, MAX> {}

impl<const MIN: u32, const MAX: u32> PartialOrd<u32> for WrappingU32<MIN, MAX> {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
    PartialOrd<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
{
    fn partial_cmp(&self, other: &WrappingU32<OTHER_MIN, OTHER_MAX>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<const MIN: u32, const MAX: u32> Ord for WrappingU32<MIN, MAX> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

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
    fn test_ord() {
        let a = WrappingU32::<0, 8>(5);
        let b = WrappingU32::<5, 20>(10);
        let c = 15;

        assert!(a < b);
        assert!(a < c);
        assert!(c > a.inner());
        assert!(c > b.inner());
    }
}
