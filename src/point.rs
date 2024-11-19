use std::ops::{Add, Mul, Sub};

use crate::traits::{IntoComponents, Roots, StdNumOps};
use crate::utils::vec_ord;
use crate::{Angle, Fraction, Zero};

/// A coordinate in a 2d space.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point<Unit> {
    /// The x-axis component.
    pub x: Unit,
    /// The y-axis component
    pub y: Unit,
}

impl<Unit> Point<Unit> {
    /// Returns a new point with the provided `x` and `y` components.
    pub const fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }

    /// Returns a new point with both `x` and `y` initialized with `i`.
    pub fn squared(i: Unit) -> Self
    where
        Unit: Clone,
    {
        Self::new(i.clone(), i)
    }

    /// Converts the contents of this point to `NewUnit` using [`From`].
    pub fn cast<NewUnit>(self) -> Point<NewUnit>
    where
        Unit: Into<NewUnit>,
    {
        Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }

    /// Converts the contents of this point to `NewUnit` using [`TryFrom`].
    ///
    /// # Errors
    ///
    /// Returns `<NewUnit as TryFrom>::Error` when the inner type cannot be
    /// converted. For this crate's types, this genenerally will be
    /// [`TryFromIntError`](std::num::TryFromIntError).
    pub fn try_cast<NewUnit>(self) -> Result<Point<NewUnit>, Unit::Error>
    where
        Unit: TryInto<NewUnit>,
    {
        Ok(Point {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }

    /// Maps each component to `map` and returns a new value with the mapped
    /// components.
    pub fn map<NewUnit>(self, mut map: impl FnMut(Unit) -> NewUnit) -> Point<NewUnit> {
        Point {
            x: map(self.x),
            y: map(self.y),
        }
    }

    /// Returns the dot product of `self` and `other`.
    #[must_use]
    pub fn dot(self, other: Point<Unit>) -> Unit
    where
        Unit: Mul<Output = Unit> + Add<Output = Unit>,
    {
        self.x * other.x + self.y * other.y
    }

    /// Returns the magnitude of self, which is the absolute distance from 0,0.
    #[must_use]
    pub fn magnitude(self) -> Unit
    where
        Unit: Mul<Output = Unit> + Add<Output = Unit> + Roots + Copy,
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns `self` rotated around `origin` by `angle`.
    #[must_use]
    pub fn rotate_around(self, origin: Point<Unit>, angle: Angle) -> Point<Unit>
    where
        Unit: Copy + Add<Output = Unit> + Sub<Output = Unit> + Mul<Fraction, Output = Unit>,
    {
        let cos = angle.cos();
        let sin = angle.sin();
        let d = self - origin;
        origin + Point::new(d.x * cos - d.y * sin, d.y * cos + d.x * sin)
    }

    /// Returns `self` rotated around `Point::ZERO` by `angle`.
    #[must_use]
    pub fn rotate_by(self, angle: Angle) -> Point<Unit>
    where
        Unit: Zero + Copy + Add<Output = Unit> + Sub<Output = Unit> + Mul<Fraction, Output = Unit>,
    {
        self.rotate_around(Self::ZERO, angle)
    }
}

impl<Unit> Ord for Point<Unit>
where
    Unit: Ord + Copy + Mul<Output = Unit>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        vec_ord::<Unit>((*self).into_components(), (*other).into_components())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}

impl<Unit> PartialOrd for Point<Unit>
where
    Unit: Ord + Copy + Mul<Output = Unit>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "euclid")]
impl<Unit> From<euclid::Point2D<f32, euclid::UnknownUnit>> for Point<Unit>
where
    Unit: crate::traits::FloatConversion<Float = f32>,
{
    fn from(point: euclid::Point2D<f32, euclid::UnknownUnit>) -> Self {
        Self {
            x: Unit::from_float(point.x),
            y: Unit::from_float(point.y),
        }
    }
}
#[cfg(feature = "euclid")]
impl<Unit> From<Point<Unit>> for euclid::Point2D<f32, euclid::UnknownUnit>
where
    Unit: crate::traits::FloatConversion<Float = f32>,
{
    fn from(point: Point<Unit>) -> Self {
        Self::new(point.x.into_float(), point.y.into_float())
    }
}

#[cfg(feature = "winit")]
impl<Unit> From<winit::dpi::PhysicalPosition<f64>> for Point<Unit>
where
    Unit: crate::traits::FloatConversion<Float = f32>,
{
    fn from(point: winit::dpi::PhysicalPosition<f64>) -> Self {
        Self {
            x: Unit::from_float(intentional::CastFrom::from_cast(point.x)),
            y: Unit::from_float(intentional::CastFrom::from_cast(point.y)),
        }
    }
}

#[cfg(feature = "winit")]
impl From<winit::dpi::PhysicalPosition<i32>> for Point<crate::units::Px> {
    fn from(point: winit::dpi::PhysicalPosition<i32>) -> Self {
        Self {
            x: crate::units::Px::new(point.x),
            y: crate::units::Px::new(point.y),
        }
    }
}

#[cfg(feature = "winit")]
impl From<winit::dpi::PhysicalPosition<u32>> for Point<crate::units::UPx> {
    fn from(point: winit::dpi::PhysicalPosition<u32>) -> Self {
        Self {
            x: crate::units::UPx::new(point.x),
            y: crate::units::UPx::new(point.y),
        }
    }
}

#[cfg(feature = "winit")]
impl From<Point<crate::units::Px>> for winit::dpi::PhysicalPosition<i32> {
    fn from(point: Point<crate::units::Px>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

#[cfg(feature = "winit")]
impl From<Point<crate::units::UPx>> for winit::dpi::PhysicalPosition<u32> {
    fn from(point: Point<crate::units::UPx>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl_2d_math!(Point, x, y);

#[cfg(feature = "wgpu")]
impl From<Point<crate::units::UPx>> for wgpu::Origin3d {
    fn from(value: Point<crate::units::UPx>) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            z: 0,
        }
    }
}

impl<T> StdNumOps for Point<T>
where
    T: StdNumOps,
{
    fn saturating_add(self, other: Self) -> Self {
        Self::new(
            self.x.saturating_add(other.x),
            self.y.saturating_add(other.y),
        )
    }

    fn saturating_mul(self, other: Self) -> Self {
        Self::new(
            self.x.saturating_mul(other.x),
            self.y.saturating_mul(other.y),
        )
    }

    fn saturating_div(self, other: Self) -> Self {
        Self::new(
            self.x.saturating_div(other.x),
            self.y.saturating_div(other.y),
        )
    }

    fn saturating_sub(self, other: Self) -> Self {
        Self::new(
            self.x.saturating_sub(other.x),
            self.y.saturating_sub(other.y),
        )
    }
}
