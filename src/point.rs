use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

use crate::traits::{FloatConversion, FromComponents, IntoComponents, Zero};

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point<Unit> {
    pub x: Unit,
    pub y: Unit,
}

impl<Unit> Point<Unit> {
    pub fn new(x: impl Into<Unit>, y: impl Into<Unit>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn cast<NewUnit>(self) -> Point<NewUnit>
    where
        NewUnit: From<Unit>,
    {
        Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }

    pub fn try_cast<NewUnit>(self) -> Option<Point<NewUnit>>
    where
        NewUnit: TryFrom<Unit>,
    {
        Some(Point {
            x: self.x.try_into().ok()?,
            y: self.y.try_into().ok()?,
        })
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

#[test]
fn add_tuple() {
    assert_eq!(Point::new(1, 2) + (3, 4), Point { x: 4, y: 6 });
}
