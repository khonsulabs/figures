use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::traits::{
    FloatConversion, FromComponents, IntoComponents, IntoSigned, IntoUnsigned, Ranged, Roots,
    ScreenScale, Zero,
};
use crate::units::{Lp, Px, UPx};
use crate::utils::vec_ord;
use crate::Round;

/// A coordinate in a 2d space.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
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
}

impl<Unit> IntoUnsigned for Point<Unit>
where
    Unit: IntoUnsigned,
{
    type Unsigned = Point<Unit::Unsigned>;

    fn into_unsigned(self) -> Self::Unsigned {
        Point {
            x: self.x.into_unsigned(),
            y: self.y.into_unsigned(),
        }
    }
}

impl<Unit> IntoSigned for Point<Unit>
where
    Unit: IntoSigned,
{
    type Signed = Point<Unit::Signed>;

    fn into_signed(self) -> Self::Signed {
        Point {
            x: self.x.into_signed(),
            y: self.y.into_signed(),
        }
    }
}

impl<Unit> Round for Point<Unit>
where
    Unit: Round,
{
    fn round(self) -> Self {
        self.map(Unit::round)
    }

    fn ceil(self) -> Self {
        self.map(Unit::ceil)
    }

    fn floor(self) -> Self {
        self.map(Unit::floor)
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

impl<Unit> ScreenScale for Point<Unit>
where
    Unit: crate::ScreenScale<Lp = Lp, Px = Px, UPx = UPx>,
{
    type Lp = Point<Lp>;
    type Px = Point<Px>;
    type UPx = Point<UPx>;

    fn into_px(self, scale: crate::Fraction) -> Self::Px {
        Point {
            x: self.x.into_px(scale),
            y: self.y.into_px(scale),
        }
    }

    fn from_px(px: Self::Px, scale: crate::Fraction) -> Self {
        Self {
            x: Unit::from_px(px.x, scale),
            y: Unit::from_px(px.y, scale),
        }
    }

    fn into_lp(self, scale: crate::Fraction) -> Self::Lp {
        Point {
            x: self.x.into_lp(scale),
            y: self.y.into_lp(scale),
        }
    }

    fn from_lp(lp: Self::Lp, scale: crate::Fraction) -> Self {
        Self {
            x: Unit::from_lp(lp.x, scale),
            y: Unit::from_lp(lp.y, scale),
        }
    }

    fn into_upx(self, scale: crate::Fraction) -> Self::UPx {
        Point {
            x: self.x.into_upx(scale),
            y: self.y.into_upx(scale),
        }
    }

    fn from_upx(px: Self::UPx, scale: crate::Fraction) -> Self {
        Self {
            x: Unit::from_upx(px.x, scale),
            y: Unit::from_upx(px.y, scale),
        }
    }
}

impl<T> FloatConversion for Point<T>
where
    T: FloatConversion,
{
    type Float = Point<T::Float>;

    fn into_float(self) -> Self::Float {
        Point {
            x: self.x.into_float(),
            y: self.y.into_float(),
        }
    }

    fn from_float(float: Self::Float) -> Self {
        Point {
            x: T::from_float(float.x),
            y: T::from_float(float.y),
        }
    }
}

#[cfg(feature = "euclid")]
impl<Unit> From<euclid::Point2D<f32, euclid::UnknownUnit>> for Point<Unit>
where
    Unit: FloatConversion<Float = f32>,
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
    Unit: FloatConversion<Float = f32>,
{
    fn from(point: Point<Unit>) -> Self {
        Self::new(point.x.into_float(), point.y.into_float())
    }
}

#[cfg(feature = "winit")]
impl<Unit> From<winit::dpi::PhysicalPosition<f64>> for Point<Unit>
where
    Unit: FloatConversion<Float = f32>,
{
    fn from(point: winit::dpi::PhysicalPosition<f64>) -> Self {
        Self {
            x: Unit::from_float(intentional::CastFrom::from_cast(point.x)),
            y: Unit::from_float(intentional::CastFrom::from_cast(point.y)),
        }
    }
}

#[cfg(feature = "winit")]
impl From<winit::dpi::PhysicalPosition<i32>> for Point<Px> {
    fn from(point: winit::dpi::PhysicalPosition<i32>) -> Self {
        Self {
            x: Px::new(point.x),
            y: Px::new(point.y),
        }
    }
}

#[cfg(feature = "winit")]
impl From<Point<Px>> for winit::dpi::PhysicalPosition<i32> {
    fn from(point: Point<Px>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T, Unit> Add<T> for Point<Unit>
where
    Unit: Add<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            x: self.x + rx,
            y: self.y + ry,
        }
    }
}

impl<T, Unit> AddAssign<T> for Point<Unit>
where
    Unit: AddAssign,
    T: IntoComponents<Unit>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl<T, Unit> Sub<T> for Point<Unit>
where
    Unit: Sub<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            x: self.x - rx,
            y: self.y - ry,
        }
    }
}

impl<T, Unit> SubAssign<T> for Point<Unit>
where
    Unit: SubAssign,
    T: IntoComponents<Unit>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl<T, Unit> Mul<T> for Point<Unit>
where
    Unit: Mul<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            x: self.x * rx,
            y: self.y * ry,
        }
    }
}

impl<T, Unit> MulAssign<T> for Point<Unit>
where
    Unit: MulAssign,
    T: IntoComponents<Unit>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.x *= rhs.0;
        self.y *= rhs.1;
    }
}

impl<T, Unit> Div<T> for Point<Unit>
where
    Unit: Div<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            x: self.x / rx,
            y: self.y / ry,
        }
    }
}

impl<T, Unit> DivAssign<T> for Point<Unit>
where
    Unit: DivAssign,
    T: IntoComponents<Unit>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.x /= rhs.0;
        self.y /= rhs.1;
    }
}

impl<Unit> Zero for Point<Unit>
where
    Unit: Zero,
{
    const ZERO: Self = Self::new(Unit::ZERO, Unit::ZERO);

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<Unit> Neg for Point<Unit>
where
    Unit: Neg<Output = Unit>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

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

impl<Unit> IntoComponents<Unit> for Point<Unit> {
    fn into_components(self) -> (Unit, Unit) {
        (self.x, self.y)
    }
}

impl<Unit> FromComponents<Unit> for Point<Unit> {
    fn from_components(components: (Unit, Unit)) -> Self {
        Self {
            x: components.0,
            y: components.1,
        }
    }
}

impl<Unit> Ranged for Point<Unit>
where
    Unit: Ranged,
{
    const MAX: Self = Self {
        x: Unit::MAX,
        y: Unit::MAX,
    };
    const MIN: Self = Self {
        x: Unit::MIN,
        y: Unit::MIN,
    };
}
