use std::ops::Mul;

use crate::{One, Scale};

/// A unit representing DPI-adjusted resolution configured on the system.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Points;

/// A unit representing physical pixels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pixels;

/// A unit representing virtual pixels that are scaled on top of the DPI
/// adjustment done for the [`Points`] unit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Scaled;

/// Scaling ratios for [`Scaled`] and [`Displayable`].
#[derive(Debug, Clone, Copy)]
pub struct DisplayScale<T> {
    pub(crate) total: Scale<T, Scaled, Pixels>,
    pub(crate) dpi: Scale<T, Points, Pixels>,
    pub(crate) additional: Scale<T, Scaled, Points>,
}

impl<T: Mul<T, Output = T> + Copy> DisplayScale<T> {
    /// Returns the scale between [`Pixels`] and [`Points`].
    pub fn dpi_scale(&self) -> Scale<T, Points, Pixels> {
        self.dpi
    }

    /// Returns the scale between [`Points`] and [`Scaled`].
    pub fn additional_scale(&self) -> Scale<T, Scaled, Points> {
        self.additional
    }

    /// Returns the scale between [`Pixels`] and [`Scaled`].
    pub fn total_scale(&self) -> Scale<T, Scaled, Pixels> {
        self.total
    }

    /// Sets the scale factor between [`Points`] and [`Scaled`].
    pub fn set_additional_scale(&mut self, scale: Scale<T, Scaled, Points>) {
        self.additional = scale;
        self.total = total_scale(self.dpi, self.additional);
    }

    /// Sets the scale factor between [`Pixels`] and [`Points`].
    pub fn set_dpi_scale(&mut self, scale: Scale<T, Points, Pixels>) {
        self.dpi = scale;
        self.total = total_scale(self.dpi, self.additional);
    }
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
        dpi: Scale<T, Points, Pixels>,
        additional_scaling: Scale<T, Scaled, Points>,
    ) -> Self {
        Self {
            dpi,
            additional: additional_scaling,
            total: total_scale(dpi, additional_scaling),
        }
    }
}

fn total_scale<T: Mul<T, Output = T> + Copy>(
    dpi: Scale<T, Points, Pixels>,
    additional_scaling: Scale<T, Scaled, Points>,
) -> Scale<T, Scaled, Pixels> {
    Scale::new(dpi.get() * additional_scaling.get())
}

impl<T> One for DisplayScale<T>
where
    T: num_traits::One + Mul<T, Output = T> + Copy,
{
    fn one() -> Self {
        Self::new(Scale::one(), Scale::one())
    }
}

/// Methods for converting between display scales.
pub trait Displayable<T> {
    /// The [`Pixels`] unit type for this implementor.
    type Pixels: Displayable<T>;
    /// The [`Points`] unit type for this implementor.
    type Points: Displayable<T>;
    /// The [`Scaled`] unit type for this implementor.
    type Scaled: Displayable<T>;

    /// Returns this value after applying `scale`, if needed.
    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels;
    /// Returns this value after applying `scale`, if needed.
    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points;
    /// Returns this value after applying `scale`, if needed.
    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled;
}
