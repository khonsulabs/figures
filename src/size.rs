use std::cmp::Ordering;
use std::ops::Mul;

use crate::traits::IntoComponents;
use crate::utils::vec_ord;
use crate::Point;

/// A width and a height measurement.
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size<Unit> {
    /// The width component
    pub width: Unit,
    /// The height component
    pub height: Unit,
}

impl<Unit> Size<Unit> {
    /// Returns a new size of the given `width` and `height`.
    pub const fn new(width: Unit, height: Unit) -> Self {
        Self { width, height }
    }

    /// Returns a new size using `dimension` for both width and height.
    pub fn squared(dimension: Unit) -> Self
    where
        Unit: Clone,
    {
        Self {
            width: dimension.clone(),
            height: dimension,
        }
    }

    /// Returns the area of the rectangle.
    pub fn area(&self) -> <Unit as Mul>::Output
    where
        Unit: Mul + Copy,
    {
        self.width * self.height
    }

    /// Converts the contents of this size to `NewUnit` using [`From`].
    pub fn cast<NewUnit>(self) -> Size<NewUnit>
    where
        NewUnit: From<Unit>,
    {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }

    /// Maps each component to `map` and returns a new value with the mapped
    /// components.
    #[must_use]
    pub fn map<NewUnit>(self, mut map: impl FnMut(Unit) -> NewUnit) -> Size<NewUnit> {
        Size {
            width: map(self.width),
            height: map(self.height),
        }
    }

    /// Converts the contents of this size to `NewUnit` using [`TryFrom`].
    ///
    /// # Errors
    ///
    /// Returns `<NewUnit as TryFrom>::Error` when the inner type cannot be
    /// converted. For this crate's types, this genenerally will be
    pub fn try_cast<NewUnit>(self) -> Result<Size<NewUnit>, NewUnit::Error>
    where
        NewUnit: TryFrom<Unit>,
    {
        Ok(Size {
            width: self.width.try_into()?,
            height: self.height.try_into()?,
        })
    }
}

impl<Unit> Ord for Size<Unit>
where
    Unit: Ord + Mul<Output = Unit> + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        vec_ord::<Unit>((*self).into_components(), (*other).into_components())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        Self {
            width: self.width.clamp(min.width, max.width),
            height: self.height.clamp(min.height, max.height),
        }
    }
}

impl<Unit> PartialOrd for Size<Unit>
where
    Unit: Ord + Mul<Output = Unit> + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl_2d_math!(Size, width, height);

impl<Unit> From<Size<Unit>> for Point<Unit> {
    fn from(value: Size<Unit>) -> Self {
        value.to_vec()
    }
}

impl<Unit> From<Point<Unit>> for Size<Unit> {
    fn from(value: Point<Unit>) -> Self {
        value.to_vec()
    }
}

#[cfg(feature = "wgpu")]
impl From<Size<crate::units::UPx>> for wgpu::Extent3d {
    fn from(value: Size<crate::units::UPx>) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
            depth_or_array_layers: 1,
        }
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

#[cfg(feature = "winit")]
impl From<winit::dpi::PhysicalSize<i32>> for Size<crate::units::Px> {
    fn from(value: winit::dpi::PhysicalSize<i32>) -> Self {
        Self {
            width: value.width.try_into().expect("width too large"),
            height: value.height.try_into().expect("height too large"),
        }
    }
}

#[cfg(feature = "winit")]
impl From<Size<crate::units::UPx>> for winit::dpi::PhysicalSize<u32> {
    fn from(size: Size<crate::units::UPx>) -> Self {
        Self {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

#[cfg(feature = "winit")]
impl From<Size<crate::units::Px>> for winit::dpi::PhysicalSize<i32> {
    fn from(size: Size<crate::units::Px>) -> Self {
        Self {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}
