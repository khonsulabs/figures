use crate::{Figure, Point};

impl<T, U> Point<T, U>
where
    T: Default + Copy,
{
    /// Returns a new point with `x`, and `T::default()` for `y`.
    pub fn from_x(x: impl Into<Figure<T, U>>) -> Self {
        Self::new(x.into().get(), T::default())
    }

    /// Returns a new point with `y`, and `T::default()` for `x`.
    pub fn from_y(y: impl Into<Figure<T, U>>) -> Self {
        Self::new(T::default(), y.into().get())
    }
}

#[test]
fn point_from_partial_tests() {
    assert_eq!(Point::<u32, ()>::from_x(1), Point::new(1, 0));
    assert_eq!(Point::<u32, ()>::from_y(1), Point::new(0, 1));
}
