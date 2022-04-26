use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct WrappingU32<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> WrappingU32<MIN, MAX> {
    pub fn new(mut inner: u32) -> Self {
        if inner > MAX {
            let rem = (inner - MIN) % (MAX - MIN);
            inner = rem + MIN;
        }

        Self(inner)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl<const MIN: u32, const MAX: u32> From<u32> for WrappingU32<MIN, MAX> {
    fn from(inner: u32) -> Self {
        Self(inner)
    }
}

impl<const MIN: u32, const MAX: u32> Add<u32> for WrappingU32<MIN, MAX> {
    type Output = u32;
    fn add(self, other: u32) -> Self::Output {
        self.0 + other
    }
}

impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
    Add<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
{
    type Output = u32;
    fn add(self, other: WrappingU32<OTHER_MIN, OTHER_MAX>) -> Self::Output {
        self.0 + other.0
    }
}

impl<const MIN: u32, const MAX: u32> AddAssign<u32> for WrappingU32<MIN, MAX> {
    fn add_assign(&mut self, other: u32) {
        *self = WrappingU32::from(*self + other);
    }
}

impl<const MIN: u32, const MAX: u32, const OTHER_MIN: u32, const OTHER_MAX: u32>
    AddAssign<WrappingU32<OTHER_MIN, OTHER_MAX>> for WrappingU32<MIN, MAX>
{
    fn add_assign(&mut self, other: WrappingU32<OTHER_MIN, OTHER_MAX>) {
        *self += other.0
    }
}

#[derive(Debug, Clone)]
pub enum ArithmeticError {
    WouldOverflow,
    WouldUnderflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let bounded = WrappingU32::<3, 9>(5);
        assert_eq!(bounded.inner(), 5)
    }

    #[test]
    fn can_add_int() {
        let bounded = WrappingU32::<3, 9>(4);
        assert_eq!(bounded + 2, 6);
    }

    #[test]
    fn can_add_other() {
        let a = WrappingU32::<3, 9>(3);
        let b = WrappingU32::<1, 302>(4);
        assert_eq!(a + b, 7);
        assert_eq!(b + a, 7);
    }
}
