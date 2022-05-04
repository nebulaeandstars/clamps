mod generic;
mod int;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundsError {
    TooSmall,
    TooLarge,
}


pub use generic::*;
pub use int::*;
