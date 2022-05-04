use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::BoundsError;

pub struct Bounded<T> {
    inner: T,
    min:   T,
    max:   T,
}

impl<
        T: PartialOrd
            + Clone
            + Add<Output = T>
            + Rem<Output = T>
            + Sub<Output = T>,
    > Bounded<T>
{
    pub fn new(inner: T, min: T, max: T) -> Result<Self, BoundsError> {
        if min >= max {
            panic!("MIN value must be less than MAX")
        }

        if inner >= max {
            Err(BoundsError::TooLarge)
        } else if inner < min {
            Err(BoundsError::TooSmall)
        } else {
            Ok(Self { inner, max, min })
        }
    }

    pub fn inner(&self) -> &T { &self.inner }
    pub fn into_inner(self) -> T { self.inner }
}

//arithmetic
macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $impl:expr) => {
        impl<T: $trait> $trait<T> for Bounded<T> {
            type Output = T::Output;
            fn $fn(self, other: T) -> Self::Output { $impl(self.inner, other) }
        }

        impl<T: $trait> $trait<Bounded<T>> for Bounded<T> {
            type Output = T::Output;
            fn $fn(self, other: Bounded<T>) -> Self::Output {
                $impl(self.inner, other.inner)
            }
        }
    };
}

impl_arith!(Add, add, |this, other| this + other);
impl_arith!(Sub, sub, |this, other| this - other);
impl_arith!(Mul, mul, |this, other| this * other);
impl_arith!(Div, div, |this, other| this / other);
impl_arith!(Rem, rem, |this, other| this % other);

// equality
impl<T: PartialEq> PartialEq<T> for Bounded<T> {
    fn eq(&self, other: &T) -> bool { self.inner == *other }
}
impl<T: PartialEq> PartialEq<Bounded<T>> for Bounded<T> {
    fn eq(&self, other: &Bounded<T>) -> bool { self.inner == other.inner }
}
impl<T: Eq> Eq for Bounded<T> {}

// We can only implement Debug if T implements Debug
impl<T: fmt::Debug> fmt::Debug for Bounded<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bounded {{inner: {:?}, min: {:?}, max: {:?}}}",
            self.inner, self.min, self.max
        )
    }
}

// We can only implement Clone if T implements Clone
impl<T: Clone> Clone for Bounded<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            min:   self.min.clone(),
            max:   self.max.clone(),
        }
    }
}
impl<T: Copy> Copy for Bounded<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let foo = Bounded::new(3, -5, 74).unwrap();
        assert_eq!(foo.into_inner(), 3)
    }

    #[test]
    fn test_debug() {
        let foo = Bounded { inner: 3, min: -5, max: 74 };
        let out = format!("{:?}", foo);
        assert_eq!(&out, "Bounded {inner: 3, min: -5, max: 74}")
    }

    #[test]
    fn test_eq() {
        let foo = Bounded { inner: 3, min: -5, max: 74 };
        let bar = Bounded { inner: 3, min: 0, max: 74 };
        assert_eq!(foo, 3);
        assert_eq!(bar, 3);
        assert_eq!(foo, bar);
    }

    #[test]
    fn test_arith() {
        let foo = Bounded::new(5.0, -100.0, 100.0).unwrap();
        for i in -10..10 {
            let num = i as f64;
            assert_eq!(foo + num, foo.inner() + num);
            assert_eq!(foo - num, foo.inner() - num);
            assert_eq!(foo * num, foo.inner() * num);

            if num != 0.0 {
                assert_eq!(foo / num, foo.inner() / num);
                assert_eq!(foo % num, foo.inner() % num);
            }
        }
    }

    #[test]
    fn cannot_create_outside_of_bounds() {
        use BoundsError::*;
        assert_eq!(Bounded::new(0.0, -10.0, 10.0).unwrap(), 0.0);
        assert_eq!(Bounded::new(-15.0, -10.0, 10.0), Err(TooSmall));
        assert_eq!(Bounded::new(15.0, -10.0, 10.0), Err(TooLarge));
    }
}
