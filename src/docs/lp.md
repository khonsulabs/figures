Logical pixels, a device-independent measurement

A logical pixel is equivalent to a single pixel on a 96-DPI display. However,
this unit is scaled when converiting to [`Px`] to honor the target display's
actual DPI.

For example, consider these assertions:

```rust
use figures::units::{Lp, Px};
use figures::{Fraction, ScreenScale};

// Scaling factor of 1.0
assert_eq!(Lp::new(1).into_px(Fraction::new_whole(1)), Px::new(1));

// Scaling factor of 2.0
assert_eq!(Lp::new(1).into_px(Fraction::new_whole(2)), Px::new(2));

// Scaling factor of 0.5
assert_eq!(Lp::new(1).into_px(Fraction::new(1, 2)), Px::from(0.5));
```

Logical pixels are designed around several use cases. Internally, this type uses
integers to represent logical pixels, which ensures that math is always
predictable, and any precision loss is controllable by the developer.

To ensure that the `Lp` type has as little precision loss as possible, it uses a
scale of 1,905 subpixels . This number may seem arbitrary, but it is a multiple
of the prime numbers 3, 5, and 127. These numbers are important because:

- 72: Typographic points are defined as 72 points per inch, and the prime
  factorization is `2^3 * 3^2`.
- 96: A scaling factor of 1 is defined as 96 pixels per inchm, and the prime
  factorization is `2^5 * 3`.
- 254: Allows using metric units lossleslly because 2.54cm = 1in, and it's prime
  factorization is `2 * 127`
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
