use std::ops::Mul;

use crate::{Figure, Size};

impl<T, Unit> Size<T, Unit>
where
    T: Mul<T, Output = T> + Copy,
{
    /// Returns the area represented by this size.
    pub fn area(&self) -> Figure<T, Unit> {
        self.width() * self.height()
    }
}

impl<T, U> Size<T, U>
where
    T: Default + Copy,
{
    /// Returns a new vector with `width`, and `T::default()` for `height`.
    pub fn from_width(width: impl Into<Figure<T, U>>) -> Self {
        Self::new(width.into().get(), T::default())
    }

    /// Returns a new vector with `height`, and `T::default()` for `width`.
    pub fn from_height(height: impl Into<Figure<T, U>>) -> Self {
        Self::new(T::default(), height.into().get())
    }
}

#[test]
fn size_from_partial_tests() {
    assert_eq!(Size::<u32, ()>::from_width(1), Size::new(1, 0));
    assert_eq!(Size::<u32, ()>::from_height(1), Size::new(0, 1));
}
