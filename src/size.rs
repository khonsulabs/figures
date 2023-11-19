use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::traits::{
    FloatConversion, FromComponents, IntoComponents, IntoSigned, IntoUnsigned, Ranged, ScreenScale,
    Zero,
};
use crate::units::{Lp, Px, UPx};
use crate::utils::vec_ord;
use crate::{Point, Round};

/// A width and a height measurement.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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
    pub fn squared(dimension: impl Into<Unit>) -> Self
    where
        Unit: Copy,
    {
        let dimension = dimension.into();
        Self {
            width: dimension,
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

impl<Unit> IntoUnsigned for Size<Unit>
where
    Unit: IntoUnsigned,
{
    type Unsigned = Size<Unit::Unsigned>;

    fn into_unsigned(self) -> Self::Unsigned {
        Size {
            width: self.width.into_unsigned(),
            height: self.height.into_unsigned(),
        }
    }
}

impl<Unit> IntoSigned for Size<Unit>
where
    Unit: IntoSigned,
{
    type Signed = Size<Unit::Signed>;

    fn into_signed(self) -> Self::Signed {
        Size {
            width: self.width.into_signed(),
            height: self.height.into_signed(),
        }
    }
}

impl<Unit> Round for Size<Unit>
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

impl<Unit> ScreenScale for Size<Unit>
where
    Unit: ScreenScale<Px = Px, Lp = Lp, UPx = UPx>,
{
    type Lp = Size<Lp>;
    type Px = Size<Px>;
    type UPx = Size<UPx>;

    fn into_px(self, scale: crate::Fraction) -> Self::Px {
        Size {
            width: self.width.into_px(scale),
            height: self.height.into_px(scale),
        }
    }

    fn from_px(px: Self::Px, scale: crate::Fraction) -> Self {
        Self {
            width: Unit::from_px(px.width, scale),
            height: Unit::from_px(px.height, scale),
        }
    }

    fn into_lp(self, scale: crate::Fraction) -> Self::Lp {
        Size {
            width: self.width.into_lp(scale),
            height: self.height.into_lp(scale),
        }
    }

    fn from_lp(lp: Self::Lp, scale: crate::Fraction) -> Self {
        Self {
            width: Unit::from_lp(lp.width, scale),
            height: Unit::from_lp(lp.height, scale),
        }
    }

    fn into_upx(self, scale: crate::Fraction) -> Self::UPx {
        Size {
            width: self.width.into_upx(scale),
            height: self.height.into_upx(scale),
        }
    }

    fn from_upx(px: Self::UPx, scale: crate::Fraction) -> Self {
        Self {
            width: Unit::from_upx(px.width, scale),
            height: Unit::from_upx(px.height, scale),
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

impl<T, Unit> Add<T> for Size<Unit>
where
    Unit: Add<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            width: self.width + rx,
            height: self.height + ry,
        }
    }
}

impl<T, Unit> AddAssign<T> for Size<Unit>
where
    Unit: AddAssign,
    T: IntoComponents<Unit>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.width += rhs.0;
        self.height += rhs.1;
    }
}

impl<T, Unit> Sub<T> for Size<Unit>
where
    Unit: Sub<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            width: self.width - rx,
            height: self.height - ry,
        }
    }
}

impl<T, Unit> SubAssign<T> for Size<Unit>
where
    Unit: SubAssign,
    T: IntoComponents<Unit>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.width -= rhs.0;
        self.height -= rhs.1;
    }
}

impl<T, Unit> Mul<T> for Size<Unit>
where
    Unit: Mul<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            width: self.width * rx,
            height: self.height * ry,
        }
    }
}

impl<T, Unit> MulAssign<T> for Size<Unit>
where
    Unit: MulAssign,
    T: IntoComponents<Unit>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.width *= rhs.0;
        self.height *= rhs.1;
    }
}

impl<T, Unit> Div<T> for Size<Unit>
where
    Unit: Div<Output = Unit>,
    T: IntoComponents<Unit>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let (rx, ry) = rhs.into_components();
        Self {
            width: self.width / rx,
            height: self.height / ry,
        }
    }
}

// impl Div<i32> for Point<UPx> {
//     type Output = Self;

//     fn div(self, rhs: i32) -> Self::Output {
//         if let Ok(rhs) = u32::try_from(rhs) {
//             Self {
//                 x: self.x / rhs,
//                 y: self.y / rhs,
//             }
//         } else {
//             Self {
//                 x: UPx::MAX,
//                 y: UPx::MAX,
//             }
//         }
//     }
// }

impl<T, Unit> DivAssign<T> for Size<Unit>
where
    Unit: DivAssign,
    T: IntoComponents<Unit>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into_components();
        self.width /= rhs.0;
        self.height /= rhs.1;
    }
}

impl<Unit> Neg for Size<Unit>
where
    Unit: Neg<Output = Unit>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            width: -self.width,
            height: -self.height,
        }
    }
}

impl<Unit> Zero for Size<Unit>
where
    Unit: Zero,
{
    const ZERO: Self = Self::new(Unit::ZERO, Unit::ZERO);

    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

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

#[cfg(feature = "winit")]
impl From<Size<crate::units::UPx>> for winit::dpi::PhysicalSize<u32> {
    fn from(size: Size<crate::units::UPx>) -> Self {
        Self {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

impl<Unit> Ranged for Size<Unit>
where
    Unit: Ranged,
{
    const MAX: Self = Self {
        width: Unit::MAX,
        height: Unit::MAX,
    };
    const MIN: Self = Self {
        width: Unit::MIN,
        height: Unit::MIN,
    };
}
