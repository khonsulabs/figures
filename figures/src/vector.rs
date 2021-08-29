use crate::{Figure, Vector};

impl<T, U> Vector<T, U>
where
    T: Default + Copy,
{
    /// Returns a new vector with `x`, and `T::default()` for `y`.
    pub fn from_x(x: impl Into<Figure<T, U>>) -> Self {
        Self::new(x.into().get(), T::default())
    }

    /// Returns a new vector with `y`, and `T::default()` for `x`.
    pub fn from_y(y: impl Into<Figure<T, U>>) -> Self {
        Self::new(T::default(), y.into().get())
    }
}

#[test]
fn vector_from_partial_tests() {
    assert_eq!(Vector::<u32, ()>::from_x(1), Vector::new(1, 0));
    assert_eq!(Vector::<u32, ()>::from_y(1), Vector::new(0, 1));
}
