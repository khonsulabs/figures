use std::{any::type_name, fmt::Debug, marker::PhantomData, ops::Mul};

use num_traits::One;

/// Allows converting between `UnitA` and `UnitB` by multiplying or dividing by
/// a scaling ratio.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale<T, UnitA, UnitB> {
    ratio: T,
    _units: PhantomData<(UnitA, UnitB)>,
}

impl<T, UnitA, UnitB> Scale<T, UnitA, UnitB> {
    /// Returns a new scale with the given `ratio`.
    pub const fn new(ratio: T) -> Self {
        Self {
            ratio,
            _units: PhantomData,
        }
    }
}

impl<T: Debug, UnitA, UnitB> Debug for Scale<T, UnitA, UnitB> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!(
            "Scale<{}, {}, {}>",
            type_name::<T>(),
            type_name::<UnitA>(),
            type_name::<UnitB>(),
        ))
        .field(&self.ratio)
        .finish()
    }
}

impl<T: Copy, UnitA, UnitB> Copy for Scale<T, UnitA, UnitB> {}
impl<T: Clone, UnitA, UnitB> Clone for Scale<T, UnitA, UnitB> {
    fn clone(&self) -> Self {
        Self::from(self.ratio.clone())
    }
}

impl<T, UnitA, UnitB> From<T> for Scale<T, UnitA, UnitB> {
    fn from(ratio: T) -> Self {
        Self {
            ratio,
            _units: PhantomData::default(),
        }
    }
}

impl<T, UnitA, UnitB> Scale<T, UnitA, UnitB>
where
    T: Copy,
{
    /// Returns the scaling ratio.
    pub fn get(&self) -> T {
        self.ratio
    }
}

impl<T, UnitA, UnitB> One for Scale<T, UnitA, UnitB>
where
    T: One,
{
    fn one() -> Self {
        Self::from(T::one())
    }
}

impl<T, UnitA, UnitB> Mul for Scale<T, UnitA, UnitB>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.ratio.mul(rhs.ratio))
    }
}
