use std::ops::Mul;

use crate::Scale;

/// A unit representing DPI-adjusted resolution configured on the system.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Points {}

/// A unit representing physical pixels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pixels {}

/// A unit representing virtual pixels that are scaled on top of the DPI
/// adjustment done for the [`Points`] unit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Scaled {}

/// Scaling ratios for [`Scaled`] and [`Displayable`].
pub struct DisplayScale<T> {
    pub(crate) scaled: Scale<T, Pixels, Scaled>,
    pub(crate) points: Scale<T, Pixels, Points>,
    pub(crate) between: Scale<T, Points, Scaled>,
}

impl<T: Mul<T, Output = T> + Copy> DisplayScale<T> {
    /// Returns a new instance with the two scales provided.
    ///
    /// * `dpi`: This scale represents the scaling between [`Pixels`] and
    ///   [`Points`]. It should be set to the system configured user interface
    ///   scaling, if possible. In general, this scale shouldn't be something an
    ///   end-user manually configures.
    /// * `additional_scaling`: This scale represents the scaling between
    ///   [`Points`] and [`Scaled`]. This is an additional layer of scaling on
    ///   top of the `dpi` scaling. It is intended to be used to provide a way
    ///   for applications to allow end-users to configure an
    ///   application-specific zoom setting.
    pub fn new(
        dpi: Scale<T, Pixels, Points>,
        additional_scaling: Scale<T, Points, Scaled>,
    ) -> Self {
        Self {
            points: dpi,
            between: additional_scaling,
            scaled: Scale::new(dpi.get() * additional_scaling.get()),
        }
    }
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
