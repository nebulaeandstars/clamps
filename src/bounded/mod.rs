//! Types that must fit within their given bounds to be constructed.

mod generic;
mod int;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundsError {
    TooSmall,
    TooLarge,
}


pub use generic::*;
pub use int::*;
