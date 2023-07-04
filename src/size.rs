use std::ops::{Div, Mul};

use crate::traits::{FloatConversion, FromComponents, IntoComponents, Zero};
use crate::Point;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Size<Unit> {
    pub width: Unit,
    pub height: Unit,
}

impl<Unit> Size<Unit> {
    pub fn new(width: impl Into<Unit>, height: impl Into<Unit>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }

    pub fn area(&self) -> <Unit as Mul>::Output
    where
        Unit: Mul + Copy,
    {
        self.width * self.height
    }

    pub fn cast<NewUnit>(self) -> Size<NewUnit>
    where
        NewUnit: From<Unit>,
    {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }

    pub fn try_cast<NewUnit>(self) -> Option<Size<NewUnit>>
    where
        NewUnit: TryFrom<Unit>,
    {
        Some(Size {
            width: self.width.try_into().ok()?,
            height: self.height.try_into().ok()?,
        })
    }
}

impl<Unit> Default for Size<Unit>
where
    Unit: Default,
{
    fn default() -> Self {
        Self {
            width: Unit::default(),
            height: Unit::default(),
        }
    }
}

impl<Unit> FloatConversion for Size<Unit>
where
    Unit: FloatConversion,
{
    type Float = Size<Unit::Float>;

    fn into_float(self) -> Self::Float {
        Size {
            width: self.width.into_float(),
            height: self.height.into_float(),
        }
    }

    fn from_float(float: Self::Float) -> Self {
        Self {
            width: Unit::from_float(float.width),
            height: Unit::from_float(float.height),
        }
    }
}

impl<Unit> Zero for Size<Unit>
where
    Unit: Zero,
{
    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl<Unit> Div<i32> for Size<Unit>
where
    Unit: Div<i32, Output = Unit>,
{
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl<Unit> Mul<i32> for Size<Unit>
where
    Unit: Mul<i32, Output = Unit>,
{
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<Unit> From<Size<Unit>> for Point<Unit> {
    fn from(value: Size<Unit>) -> Self {
        value.to()
    }
}

impl<Unit> From<Point<Unit>> for Size<Unit> {
    fn from(value: Point<Unit>) -> Self {
        value.to()
    }
}

#[cfg(feature = "wgpu")]
impl From<Size<crate::units::UPx>> for wgpu::Extent3d {
    fn from(value: Size<crate::units::UPx>) -> Self {
        Self {
            width: value.width.0,
            height: value.height.0,
            depth_or_array_layers: 1,
        }
    }
}

impl<Unit> FromComponents<Unit> for Size<Unit> {
    fn from_components(components: (Unit, Unit)) -> Self {
        Self {
            width: components.0,
            height: components.1,
        }
    }
}

impl<Unit> IntoComponents<Unit> for Size<Unit> {
    fn into_components(self) -> (Unit, Unit) {
        (self.width, self.height)
    }
}

#[cfg(feature = "winit")]
impl From<winit::dpi::PhysicalSize<u32>> for Size<crate::units::UPx> {
    fn from(value: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: value.width.try_into().expect("width too large"),
            height: value.height.try_into().expect("height too large"),
        }
    }
}
