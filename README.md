# clamps

Wrapping, saturating, and other forms of number clamping!

Doing this sucks:
```rust
if foo >= min && foo <= max {
    // do stuff
}
```

Instead, it'd be great if we could encode `max` and `min` into the *type* of
`foo` somehow, so that we can get all the benefits of bounds checking without
having to hard-code the busy-work ourselves. Ada has a nice solution with its
[range](https://en.m.wikibooks.org/wiki/Ada_Programming/Types/range) and
[mod](https://en.m.wikibooks.org/wiki/Ada_Programming/Types/mod) types, and it
would be great if Rust had them, too.

### What's in this crate?

Clamps provides a variety of types with three basic forms:
- `Bounded`
    - Can't be constructed from an out-of-bounds value.
- `Wrapping`
    - Will wrap to fit within bounds when constructed from a value.
    - Similar to Ada's
      [mod](https://en.m.wikibooks.org/wiki/Ada_Programming/Types/mod) type.
    - Implements `AddAssign`, `SubAssign`, etc.
- `Saturating`
    - Will "saturate" to fit within bounds when constructed from a value:
        - Values that are too large will be set to MAX.
        - Values that are too small will be set to MIN.
    - Implements `AddAssign`, `SubAssign`, etc.

All three variants have generic and concrete forms. The generic types are
slightly more expensive and cumbersome, but can be used with more types. The
concrete forms are cheap and easier to use, but are currently limited to
integers.

The generic variants also allow you to assign their bounds at runtime, which can
help when dynamically checking indexes, etc.
