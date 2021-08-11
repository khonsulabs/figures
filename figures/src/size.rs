use std::ops::Mul;

use crate::Figure;

impl<T, Unit> crate::Size<T, Unit>
where
    T: Mul<T, Output = T> + Copy,
{
    /// Returns the area represented by this size.
    pub fn area(&self) -> Figure<T, Unit> {
        self.width() * self.height()
    }
}
