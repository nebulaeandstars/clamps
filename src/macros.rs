// provides an easier way to define two implementations:
// - impl $trait<u32> for WrappingT<MIN, MAX>
// - impl $trait<WrappingT<OTHER_MIN, OTHER_MAX>> for WrappingT<MIN, MAX>
//
// The generics were getting a bit ridiculous, so this short-hand exists to
// make defining arithmetic traits like Add, Sub, etc. much easier.
macro_rules! impl_arith {
    ($type:ty, $other:ty, $inner:ty, $trait:ident, $fn:ident, $impl:expr) => {
        impl<const MIN: $inner, const MAX: $inner> $trait<$inner> for $type {
            type Output = $inner;
            fn $fn(self, other: $inner) -> Self::Output { $impl(self.0, other) }
        }

        impl<
                const MIN: $inner,
                const MAX: $inner,
                const OTHER_MIN: $inner,
                const OTHER_MAX: $inner,
            > $trait<$other> for $type
        {
            type Output = $inner;
            fn $fn(self, other: $other) -> Self::Output {
                $impl(self.0, other.0)
            }
        }
    };
}
pub(crate) use impl_arith;

macro_rules! impl_arith_assign {
    ($type:ty, $other:ty, $inner:ty, $trait:ident, $fn:ident, $op:ident) => {
        impl<const MIN: $inner, const MAX: $inner> $trait<$inner> for $type {
            fn $fn(&mut self, other: $inner) { *self = self.$op(other).into() }
        }

        impl<
                const MIN: $inner,
                const MAX: $inner,
                const OTHER_MIN: $inner,
                const OTHER_MAX: $inner,
            > $trait<$other> for $type
        {
            fn $fn(&mut self, other: $other) { *self = self.$op(other).into() }
        }
    };
}
pub(crate) use impl_arith_assign;

macro_rules! impl_ord {
    ($type:ty, $other:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> PartialEq<$inner> for $type {
            fn eq(&self, other: &$inner) -> bool { self.0 == *other }
        }

        impl<
                const MIN: $inner,
                const MAX: $inner,
                const OTHER_MIN: $inner,
                const OTHER_MAX: $inner,
            > PartialEq<$other> for $type
        {
            fn eq(&self, other: &$other) -> bool { self.0 == other.0 }
        }
        impl<const MIN: $inner, const MAX: $inner> Eq for $type {}

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$inner>
            for $type
        {
            fn partial_cmp(&self, other: &$inner) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl<
                const MIN: $inner,
                const MAX: $inner,
                const OTHER_MIN: $inner,
                const OTHER_MAX: $inner,
            > PartialOrd<$other> for $type
        {
            fn partial_cmp(&self, other: &$other) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl<const MIN: $inner, const MAX: $inner> Ord for $type {
            fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
        }
    };
}
pub(crate) use impl_ord;
