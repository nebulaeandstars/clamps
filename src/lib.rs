//! Wrapping, saturating, and other forms of number clamping!
//!
//! This crate takes inspiration from Ada's
//! [range](https://en.m.wikibooks.org/wiki/Ada_Programming/Types/range) and
//! [mod](https://en.m.wikibooks.org/wiki/Ada_Programming/Types/mod) types,
//! providing type-level abstractions that can be used to clamp values into
//! a given set of bounds.
//!
//! The basic idea is that the following sucks, and we can do better:
//!
//! ```rust
//! # let foo = 0;
//! # let min = 0;
//! # let max = 0;
//! if foo >= min && foo < max {
//!     // do stuff
//! }
//! ```
//!
//! # What this crate does
//!
//! Clamps provides three main forms of clamping:
//! - Wrapping types (will wrap to fit into range)
//! - Saturating types (will saturate to fit into range)
//! - Bounded types (can't be constructed at all when out of range)
//!
//! These in turn have two variants:
//! - The generic `Wrapping<T>`, `Saturating<T>`, and `Bounded<T>` types:
//!   - Can be used with any compatible type.
//!   - Can be assigned bounds at runtime.
//!   - More expensive as the bounds have to be contained within the type
//!   itself.
//! - Concrete types such as `WrappingUSize<MIN, MAX>`:
//!   - Cheaper and more ergonimic.
//!   - Bounds are `const` and must be set at compile time.
//!   - Only available for integers.
//!
//! # Examples
//!
//! Bounded types:
//! ```rust
//! use clamps::bounded::{BoundedU32, BoundsError};
//!
//! let bounded = BoundedU32::<3, 7>::try_from(5);
//! assert_eq!(bounded.unwrap(), 5);
//!
//! let invalid = BoundedU32::<3, 7>::try_from(10);
//! assert_eq!(invalid, Err(BoundsError::TooLarge));
//! ```
//!
//! Wrapping types:
//! ```rust
//! use clamps::wrapping::WrappingU32;
//!
//! let mut wrapping = WrappingU32::<2, 8>::from(5);
//! assert_eq!(wrapping, 5);
//! assert_eq!(wrapping + 5, 10);
//!
//! // 5 + 5 (bounds: 2..8) = 4
//! wrapping += 5;
//! assert_ne!(wrapping, 10);
//! assert_eq!(wrapping, 4);
//!
//! // 4 - 3 (bounds: 2..8) = 7
//! wrapping -= 3;
//! assert_ne!(wrapping, 1);
//! assert_eq!(wrapping, 7);
//! ```
//!
//! Saturating types:
//! ```rust
//! use clamps::saturating::SaturatingU32;
//!
//! let mut saturating = SaturatingU32::<5, 10>::from(8);
//! assert_eq!(saturating, 8);
//! assert_eq!(saturating + 5, 13);
//!
//! // 8 + 100 (bounds: 5..=10) = 10
//! saturating += 100;
//! assert_eq!(saturating, 10);
//!
//! // 10 - 100 (bounds: 5..=10) = 5
//! saturating -= 100;
//! assert_eq!(saturating, 5);
//! ```


pub mod bounded;
mod macros;
pub mod saturating;
pub mod wrapping;
