use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::traits::{
    FloatConversion, FromComponents, IntoComponents, IntoSigned, IntoUnsigned, IsZero, ScreenScale,
};
use crate::units::{Lp, Px};
use crate::utils::vec_ord;

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

    /// Converts the contents of this point to `NewUnit` using [`From`].
    pub fn cast<NewUnit>(self) -> Point<NewUnit>
    where
        NewUnit: From<Unit>,
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
    pub fn try_cast<NewUnit>(self) -> Result<Point<NewUnit>, NewUnit::Error>
    where
        NewUnit: TryFrom<Unit>,
    {
        Ok(Point {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
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
    Unit: ScreenScale<Px = Px, Lp = Lp>,
{
    type Lp = Point<Lp>;
    type Px = Point<Px>;

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

impl<Unit> IsZero for Point<Unit>
where
    Unit: IsZero,
{
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
            x: value.x.0,
            y: value.y.0,
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
