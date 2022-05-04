//! Tests to validate that types work alongside each-other as expected.

use clamps::*;

#[test]
fn wrapping_t_is_interoperable_with_t() {
    let mut generic = Wrapping::new(3, -5, 10);
    let mut concrete = WrappingI32::<-5, 10>::new(3);

    generic += 1;
    concrete -= 1;
    assert_eq!(generic, 4);
    assert_eq!(concrete, 2);

    assert_eq!(generic * 2 % 5, 3);
    assert_eq!(concrete / 2, 1);
}
