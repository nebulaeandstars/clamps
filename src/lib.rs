use std::ops::Add;
use std::ops::Deref;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

pub struct BoundedU32<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> BoundedU32<MIN, MAX> {
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }

    pub fn is_valid(&self) -> bool {
        self.0 >= MIN && self.0 < MAX
    }
}

impl<const MIN: u32, const MAX: u32> From<u32> for BoundedU32<MIN, MAX> {
    fn from(inner: u32) -> Self {
        Self(inner)
    }
}

impl<const MIN: u32, const MAX: u32> Add<u32> for BoundedU32<MIN, MAX> {
    type Output = Result<BoundedU32<MIN, MAX>, ArithmeticError>;
    fn add(self, other: u32) -> Self::Output {
        let result = BoundedU32::from(self.0 + other);
        if result.is_valid() {
            Ok(result)
        } else {
            Err(ArithmeticError::WouldOverflow)
        }
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
        let bounded = BoundedU32::<3, 9>(5);
        assert!(bounded.is_valid());
    }

    #[test]
    fn can_add() {
        let bounded = BoundedU32::<3, 9>(5);
        let result = bounded + 2;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 7);
    }
}
