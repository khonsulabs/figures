/// Allows comparing floating point numbers with approximation.
pub trait Approx<T: approx::AbsDiffEq> {
    /// Returns true if the values are approximately equal. Uses
    /// [`approx::AbsDiffEq`] with the default epsilon.
    fn approx_eq(&self, other: &Self) -> bool;
}
