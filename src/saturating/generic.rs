use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

pub struct Saturating<T> {
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
    > Saturating<T>
{
    pub fn new(mut inner: T, min: T, max: T) -> Self {
        if min >= max {
            panic!("MIN value must be less than MAX")
        }

        if inner >= max {
            inner = max.clone()
        } else if inner < min {
            inner = min.clone()
        }

        Self { inner, max, min }
    }

    pub fn inner(&self) -> &T { &self.inner }
    pub fn into_inner(self) -> T { self.inner }
}

//arithmetic
macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $impl:expr) => {
        impl<T: $trait> $trait<T> for Saturating<T> {
            type Output = T::Output;
            fn $fn(self, other: T) -> Self::Output { $impl(self.inner, other) }
        }

        impl<T: $trait> $trait<Saturating<T>> for Saturating<T> {
            type Output = T::Output;
            fn $fn(self, other: Saturating<T>) -> Self::Output {
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

// assigning arithmetic
macro_rules! impl_arith_assign {
    ($trait:ident, $fn:ident, $impl:expr) => {
        impl<
                T: PartialOrd
                    + Copy
                    + Add<Output = T>
                    + Sub<Output = T>
                    + Mul<Output = T>
                    + Div<Output = T>
                    + Rem<Output = T>
                    + $trait,
            > $trait<T> for Saturating<T>
        {
            fn $fn(&mut self, other: T) {
                *self = Saturating::new($impl(*self, other), self.min, self.max)
            }
        }

        impl<
                T: PartialOrd
                    + Copy
                    + Add<Output = T>
                    + Sub<Output = T>
                    + Mul<Output = T>
                    + Div<Output = T>
                    + Rem<Output = T>
                    + $trait,
            > $trait<Saturating<T>> for Saturating<T>
        {
            fn $fn(&mut self, other: Saturating<T>) {
                *self = Saturating::new($impl(*self, other), self.min, self.max)
            }
        }
    };
}

impl_arith_assign!(AddAssign, add_assign, |this, other| this + other);
impl_arith_assign!(SubAssign, sub_assign, |this, other| this - other);
impl_arith_assign!(MulAssign, mul_assign, |this, other| this * other);
impl_arith_assign!(DivAssign, div_assign, |this, other| this / other);
impl_arith_assign!(RemAssign, rem_assign, |this, other| this % other);

// equality
impl<T: PartialEq> PartialEq<T> for Saturating<T> {
    fn eq(&self, other: &T) -> bool { self.inner == *other }
}
impl<T: PartialEq> PartialEq<Saturating<T>> for Saturating<T> {
    fn eq(&self, other: &Saturating<T>) -> bool { self.inner == other.inner }
}
impl<T: Eq> Eq for Saturating<T> {}

// We can only implement Debug if T implements Debug
impl<T: fmt::Debug> fmt::Debug for Saturating<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Saturating {{inner: {:?}, min: {:?}, max: {:?}}}",
            self.inner, self.min, self.max
        )
    }
}

// We can only implement Clone if T implements Clone
impl<T: Clone> Clone for Saturating<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            min:   self.min.clone(),
            max:   self.max.clone(),
        }
    }
}
impl<T: Copy> Copy for Saturating<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let foo = Saturating::new(3, -5, 74);
        assert_eq!(foo.into_inner(), 3)
    }

    #[test]
    fn test_debug() {
        let foo = Saturating { inner: 3, min: -5, max: 74 };
        let out = format!("{:?}", foo);
        assert_eq!(&out, "Saturating {inner: 3, min: -5, max: 74}")
    }

    #[test]
    fn test_eq() {
        let foo = Saturating { inner: 3, min: -5, max: 74 };
        let bar = Saturating { inner: 3, min: 0, max: 74 };
        assert_eq!(foo, 3);
        assert_eq!(bar, 3);
        assert_eq!(foo, bar);
    }

    #[test]
    fn test_arith() {
        let foo = Saturating::new(5.0, -100.0, 100.0);
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
    fn test_saturating() {
        let mut foo = Saturating::new(5.0, 0.0, 10.0);
        foo -= 10.0;
        assert_eq!(foo, 0.0);
        foo -= 100.0;
        assert_eq!(foo, 0.0);
        foo += 100.0;
        assert_eq!(foo, 10.0);
    }
}
