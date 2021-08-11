use crate::Scale;

/// A unit representing DPI-adjusted resolution configured on the system.
pub enum Points {}

/// A unit representing physical pixels.
pub enum Pixels {}

/// A unit representing virtual pixels that are scaled on top of the DPI
/// adjustment done for the [`Points`] unit.
pub enum Scaled {}

/// Scaling ratios for [`Scaled`] and [`Displayable`].
pub struct DisplayScale<T> {
    pub(crate) scaled: Scale<T, Pixels, Scaled>,
    pub(crate) points: Scale<T, Pixels, Points>,
    pub(crate) between: Scale<T, Points, Scaled>,
}

/// Converts from [`Points`] or [`Pixels`] units to [`Scaled`].
pub trait ToScaled<T> {
    /// The [`Scaled`] unit type for this implementor.
    type Scaled;

    /// Returns this value after applying `scale`, if needed.
    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled;
}

/// Converts from [`Points`] or [`Pixels`] units to [`Points`].
pub trait ToPoints<T> {
    /// The [`Points`] unit type for this implementor.
    type Points;

    /// Returns this value after applying `scale`, if needed.
    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points;
}

/// Converts from [`Points`] or [`Scaled`] units to [`Pixels`].
pub trait ToPixels<T> {
    /// The [`Pixels`] unit type for this implementor.
    type Pixels;

    /// Returns this value after applying `scale`, if needed.
    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels;
}

/// Methods for converting between display scales.
pub trait Displayable<T>: ToScaled<T> + ToPoints<T> + ToPixels<T> {}

impl<T, D> Displayable<T> for D where D: ToScaled<T> + ToPoints<T> + ToPixels<T> {}
