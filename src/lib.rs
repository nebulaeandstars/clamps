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


pub mod bounded;
mod macros;
pub mod saturating;
pub mod wrapping;
