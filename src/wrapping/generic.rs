use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Rem, RemAssign, Sub,
    SubAssign,
};

pub struct Wrapping<T> {
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
    > Wrapping<T>
{
    pub fn new(mut inner: T, min: T, max: T) -> Self {
        if min >= max {
            panic!("MIN value must be less than MAX")
        }

        // TODO: Reduce the number of clones needed here
        if inner >= max {
            let rem = (inner - min.clone()) % (max.clone() - min.clone());
            inner = min.clone() + rem;
        } else if inner < min {
            let rem =
                (inner.clone() + min.clone()) % (max.clone() - min.clone());
            inner = min.clone() + rem;
        }

        Self { inner, max, min }
    }

    pub fn inner(&self) -> &T { &self.inner }
    pub fn into_inner(self) -> T { self.inner }
    pub fn range(&self) -> Range<&T> { &self.min..&self.max }
    pub fn min_bound(&self) -> &T { &self.min }
    pub fn max_bound(&self) -> &T { &self.max }
}

//arithmetic
macro_rules! impl_arith {
    ($trait:ident, $fn:ident, $impl:expr) => {
        impl<T: $trait> $trait<T> for Wrapping<T> {
            type Output = T::Output;
            fn $fn(self, other: T) -> Self::Output { $impl(self.inner, other) }
        }

        impl<T: $trait> $trait<Wrapping<T>> for Wrapping<T> {
            type Output = T::Output;
            fn $fn(self, other: Wrapping<T>) -> Self::Output {
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
            > $trait<T> for Wrapping<T>
        {
            fn $fn(&mut self, other: T) {
                *self = Wrapping::new($impl(*self, other), self.min, self.max)
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
            > $trait<Wrapping<T>> for Wrapping<T>
        {
            fn $fn(&mut self, other: Wrapping<T>) {
                *self = Wrapping::new($impl(*self, other), self.min, self.max)
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
impl<T: PartialEq> PartialEq<T> for Wrapping<T> {
    fn eq(&self, other: &T) -> bool { self.inner == *other }
}
impl<T: PartialEq> PartialEq<Wrapping<T>> for Wrapping<T> {
    fn eq(&self, other: &Wrapping<T>) -> bool { self.inner == other.inner }
}
impl<T: Eq> Eq for Wrapping<T> {}

// We can only implement Debug if T implements Debug
impl<T: fmt::Debug> fmt::Debug for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wrapping {{inner: {:?}, min: {:?}, max: {:?}}}",
            self.inner, self.min, self.max
        )
    }
}

// We can only implement Clone if T implements Clone
impl<T: Clone> Clone for Wrapping<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            min:   self.min.clone(),
            max:   self.max.clone(),
        }
    }
}
impl<T: Copy> Copy for Wrapping<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let foo = Wrapping::new(3, -5, 74);
        assert_eq!(foo.into_inner(), 3)
    }

    #[test]
    fn test_debug() {
        let foo = Wrapping { inner: 3, min: -5, max: 74 };
        let out = format!("{:?}", foo);
        assert_eq!(&out, "Wrapping {inner: 3, min: -5, max: 74}")
    }

    #[test]
    fn test_max_and_min() {
        let foo = Wrapping::new(4, -3, 8);
        assert_eq!(foo.min_bound(), &-3);
        assert_eq!(foo.max_bound(), &8);
        assert_eq!(foo.range(), &-3..&8);
    }

    #[test]
    fn test_eq() {
        let foo = Wrapping { inner: 3, min: -5, max: 74 };
        let bar = Wrapping { inner: 3, min: 0, max: 74 };
        assert_eq!(foo, 3);
        assert_eq!(bar, 3);
        assert_eq!(foo, bar);
    }

    #[test]
    fn test_arith() {
        let foo = Wrapping::new(5.0, -100.0, 100.0);
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
    fn test_wrapping() {
        let mut foo = Wrapping::new(0.0, 0.0, 10.0);
        for i in 0..1000 {
            let num = i as f64 / 2.0;

            assert_eq!(foo, num % 10.0);
            assert_eq!(Wrapping::new(num, 0.0, 10.0), foo);

            foo += 0.5;
        }
    }
}
