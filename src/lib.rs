use std::ops::Add;
use std::ops::AddAssign;

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
        *self = WrappingU32::from(self.0 + other);
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

impl<const MIN: u32, const MAX: u32> PartialEq for WrappingU32<MIN, MAX> {
    fn eq(&self, other: &WrappingU32<MIN, MAX>) -> bool {
        self.0 == other.0
    }
}
impl<const MIN: u32, const MAX: u32> Eq for WrappingU32<MIN, MAX> {}

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
    fn add_assign_matches_new() {
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
}
