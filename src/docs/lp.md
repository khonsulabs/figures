Logical pixels, a device-independent measurement

Logical pixels are designed around several use cases. Internally, this type uses
integers to represent logical pixels, which ensures that math is always
predictable, and any precision loss is controllable by the developer.

To ensure that the `Lp` type has as little precision loss as possible, it uses a
scale of 36,576 pixels per inch. This number may seem arbitrary, but it is the
lowest common multiple of:

- 72: Typographic points are defined as 72 points per inch
- 96: A scaling factor of 1 is defined as 96 pixels per inch
- 254: Allows using metric units lossleslly because 2.54cm = 1in
- 5: Five is a low common prime, and everyone likes round numbers.

Because the `Lp` scale is arbitrary and hard to reason about, this type has many
constructors to allow specifying measurements in various units developers will
be more comfortable with:

```rust
use figures::units::Lp;

// Centimeters
assert_eq!(Lp::cm_f(1.), Lp::cm(1));
// Millimeters
assert_eq!(Lp::mm_f(1.), Lp::mm(1));
assert_eq!(Lp::mm(10), Lp::cm(1));

// Inches
assert_eq!(Lp::inches_f(1.), Lp::inches(1));
assert_eq!(Lp::inches(1), Lp::cm_f(2.54));

// Points
assert_eq!(Lp::points_f(36.), Lp::points(36));
assert_eq!(Lp::points(36), Lp::inches_f(0.5));
```
