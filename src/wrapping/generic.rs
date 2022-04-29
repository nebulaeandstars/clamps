use std::fmt;
use std::ops::{Add, Rem, Sub};

pub struct Wrapping<T> {
    inner: T,
    min:   T,
    max:   T,
}

impl<T: Ord + Clone + Add<Output = T> + Rem<Output = T> + Sub<Output = T>>
    Wrapping<T>
{
    fn new(mut inner: T, min: T, max: T) -> Self {
        if min >= max {
            panic!("MIN value must be less than MAX")
        }

        // TODO: Reduce the number of clones needed here
        if inner >= max {
            let rem = (inner - min.clone()) % (max.clone() - min.clone());
            inner = min.clone() + rem.clone();
        } else if inner < min {
            let rem =
                (inner.clone() + min.clone()) % (max.clone() - min.clone());
            inner = min.clone() + rem.clone();
        }

        Self { inner, max, min }
    }

    fn inner(&self) -> &T { &self.inner }
    fn into_inner(self) -> T { self.inner }
}

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
}