/// Allows comparing floating point numbers with approximation.
///
/// Uses [`approx::AbsDiffEq`] with the default epsilon.
pub trait Approx<T: approx::AbsDiffEq> {
    /// Returns true if the values are approximately equal. Uses
    /// [`approx::AbsDiffEq`] with the default epsilon.
    fn approx_eq(&self, other: &Self) -> bool;

    /// Returns true if the values are not approximately equal. Uses
    /// [`approx::AbsDiffEq`] with the default epsilon.
    fn approx_ne(&self, other: &Self) -> bool {
        !self.approx_eq(other)
    }
}
