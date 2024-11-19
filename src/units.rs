use std::cmp::Ordering;
use std::fmt;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use intentional::{Cast, CastFrom};

use crate::traits::{
    Abs, FloatConversion, IntoComponents, IntoSigned, IntoUnsigned, Pow, Roots, Round, ScreenScale,
    StdNumOps, UnscaledUnit, Zero,
};
use crate::Fraction;

pub(crate) const ARBITRARY_SCALE: u16 = 1905;
const ARBITRARY_SCALE_I32: i32 = ARBITRARY_SCALE as i32;
const ARBITRARY_SCALE_U32: u32 = ARBITRARY_SCALE as u32;
#[allow(clippy::cast_precision_loss)]
const ARBITRARY_SCALE_F32: f32 = ARBITRARY_SCALE as f32;

macro_rules! define_integer_type {
    ($name:ident, $inner:ty, $docs_file:literal, $scale:literal) => {
        #[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[doc = include_str!($docs_file)]
        #[repr(C)]
        pub struct $name($inner);

        impl $name {
            /// The maximum value for this type.
            pub const MAX: Self = Self(<$inner>::MAX);
            /// The minimum value for this type.
            pub const MIN: Self = Self(<$inner>::MIN);

            /// Returns a new wrapped value for this unit.
            #[must_use]
            pub const fn new(value: $inner) -> Self {
                Self(value * $scale)
            }

            /// Returns the contained value, rounded if applicable.
            #[must_use]
            pub const fn get(self) -> $inner {
                if $scale > 1 {
                    (self.0 + $scale / 2) / $scale
                } else {
                    self.0
                }
            }

            /// Returns the result of subtracting `other` from `self`. If the
            /// calculation overflows, the value will be limited to
            /// [`Self::MIN`]/[`Self::MAX`].
            #[must_use]
            pub const fn saturating_sub(self, other: Self) -> Self {
                Self(self.0.saturating_sub(other.0))
            }

            /// Returns the result of adding `self` and `other`. If the
            /// calculation overflows, the value will be limited to
            /// [`Self::MIN`]/[`Self::MAX`].
            #[must_use]
            pub const fn saturating_add(self, other: Self) -> Self {
                Self(self.0.saturating_add(other.0))
            }

            /// Returns the result of multiplying `self` and `other`. If the
            /// calculation overflows, the value will be limited to
            /// [`Self::MIN`]/[`Self::MAX`].
            #[must_use]
            pub const fn saturating_mul(self, other: Self) -> Self {
                Self(self.0.saturating_mul(other.0) / $scale)
            }

            /// Returns the result of dividing `self` by `other`. If the
            /// calculation overflows, the value will be limited to
            /// [`Self::MIN`]/[`Self::MAX`].
            #[must_use]
            pub const fn saturating_div(self, other: Self) -> Self {
                Self::new(self.0.saturating_div(other.0))
            }
        }

        impl FloatConversion for $name {
            type Float = f32;

            #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
            fn into_float(self) -> Self::Float {
                self.0.cast::<f32>() / $scale.cast::<f32>()
            }

            fn from_float(float: Self::Float) -> Self {
                Self((float * $scale.cast::<f32>()).round().cast())
            }
        }

        impl From<$name> for f32 {
            fn from(value: $name) -> Self {
                value.into_float()
            }
        }

        impl From<f32> for $name {
            fn from(value: f32) -> Self {
                Self::from_float(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }

        impl PartialEq<$inner> for $name {
            fn eq(&self, other: &$inner) -> bool {
                self == &Self::new(*other)
            }
        }

        impl PartialOrd<$inner> for $name {
            fn partial_cmp(&self, other: &$inner) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&Self::new(*other))
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.0 / rhs.0)
            }
        }

        impl Div<$inner> for $name {
            type Output = Self;

            fn div(self, rhs: $inner) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl Div<f32> for $name {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self::from((self.into_float() / rhs))
            }
        }

        impl Div<Fraction> for $name {
            type Output = Self;

            fn div(self, rhs: Fraction) -> Self::Output {
                Self::from_unscaled((self.into_unscaled() / rhs))
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs
            }
        }

        impl DivAssign<$inner> for $name {
            fn div_assign(&mut self, rhs: $inner) {
                self.0 /= rhs;
            }
        }

        impl Rem for $name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl Rem<$inner> for $name {
            type Output = Self;

            fn rem(self, rhs: $inner) -> Self::Output {
                Self(self.0 % rhs)
            }
        }

        impl RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        impl RemAssign<$inner> for $name {
            fn rem_assign(&mut self, rhs: $inner) {
                self.0 %= rhs;
            }
        }

        impl Rem<f32> for $name {
            type Output = Self;

            fn rem(self, rhs: f32) -> Self::Output {
                Self::from((self.into_float() % rhs).round())
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0 / $scale)
            }
        }

        impl Mul<$inner> for $name {
            type Output = Self;

            fn mul(self, rhs: $inner) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl Mul<f32> for $name {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::from((self.into_float() * rhs))
            }
        }

        impl Mul<Fraction> for $name {
            type Output = Self;

            fn mul(self, rhs: Fraction) -> Self::Output {
                Self::from_unscaled((self.into_unscaled() * rhs))
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 = (self.0 * rhs.0) / $scale;
            }
        }

        impl MulAssign<$inner> for $name {
            fn mul_assign(&mut self, rhs: $inner) {
                self.0 *= rhs;
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Add<$inner> for $name {
            type Output = Self;

            fn add(self, rhs: $inner) -> Self::Output {
                self + Self::new(rhs)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl AddAssign<$inner> for $name {
            fn add_assign(&mut self, rhs: $inner) {
                *self += Self::new(rhs);
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl Sub<$inner> for $name {
            type Output = Self;

            fn sub(self, rhs: $inner) -> Self::Output {
                Self(self.0 - rhs * $scale)
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl SubAssign<$inner> for $name {
            fn sub_assign(&mut self, rhs: $inner) {
                *self -= Self::new(rhs);
            }
        }

        impl Zero for $name {
            const ZERO: Self = Self(0);

            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }

        impl UnscaledUnit for $name {
            type Representation = $inner;

            fn from_unscaled(unscaled: Self::Representation) -> Self {
                Self(unscaled)
            }

            fn into_unscaled(self) -> Self::Representation {
                self.0
            }
        }

        impl Round for $name {
            fn round(self) -> Self {
                Self((self.0 + $scale / 2) / $scale * $scale)
            }

            fn ceil(self) -> Self {
                Self((self.0 + $scale - 1) / $scale * $scale)
            }

            fn floor(self) -> Self {
                Self(self.0 / $scale * $scale)
            }
        }

        impl Roots for $name {
            fn sqrt(self) -> Self {
                Self(f64::from(self.0).sqrt().cast())
            }

            fn cbrt(self) -> Self {
                Self(f64::from(self.0).cbrt().cast())
            }
        }

        impl StdNumOps for $name {
            fn saturating_add(self, other: Self) -> Self {
                self.saturating_add(other)
            }

            fn saturating_mul(self, other: Self) -> Self {
                self.saturating_mul(other)
            }

            fn saturating_div(self, other: Self) -> Self {
                self.saturating_div(other)
            }

            fn saturating_sub(self, other: Self) -> Self {
                self.saturating_sub(other)
            }
        }
    };
}

impl CastFrom<f32> for Px {
    fn from_cast(from: f32) -> Self {
        Px::from(from)
    }
}

impl CastFrom<Px> for f32 {
    fn from_cast(from: Px) -> Self {
        from.into_float()
    }
}

define_integer_type!(Lp, i32, "docs/lp.md", 1905);

impl IntoComponents<Lp> for i32 {
    fn into_components(self) -> (Lp, Lp) {
        (Lp(self), Lp(self))
    }
}

impl IntoComponents<Lp> for f32 {
    fn into_components(self) -> (Lp, Lp) {
        let value = Lp::from_float(self);
        (value, value)
    }
}

impl ScreenScale for Lp {
    type Lp = Lp;
    type Px = Px;
    type UPx = UPx;

    fn into_px(self, scale: Fraction) -> Self::Px {
        Px(self.0 * 4 * scale / ARBITRARY_SCALE_I32)
    }

    fn from_px(px: Self::Px, scale: Fraction) -> Self {
        px.into_lp(scale)
    }

    fn into_lp(self, _scale: Fraction) -> Self::Lp {
        self
    }

    fn from_lp(lp: Self::Lp, _scale: Fraction) -> Self {
        lp
    }

    fn into_upx(self, scale: crate::Fraction) -> Self::UPx {
        self.into_px(scale).into_unsigned()
    }

    fn from_upx(px: Self::UPx, scale: crate::Fraction) -> Self {
        Self::from_px(px.into_signed(), scale)
    }
}

impl std::ops::Neg for Lp {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl TryFrom<u32> for Lp {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl Lp {
    /// Returns a value equivalent to the number of `points` provided. One
    /// [point](https://en.wikipedia.org/wiki/Point_(typography)) is 1/72 of an
    /// inch.
    #[must_use]
    pub const fn points(points: i32) -> Self {
        Self(points * ARBITRARY_SCALE_I32 * 4 / 3)
    }

    /// Returns a value equivalent to the number of `points` provided. One
    /// [point](https://en.wikipedia.org/wiki/Point_(typography)) is 1/72 of an
    /// inch.
    #[must_use]
    pub fn points_f(points: f32) -> Self {
        Lp((points * ARBITRARY_SCALE_F32 * 4. / 3.).cast())
    }

    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub const fn cm(centimeters: i32) -> Self {
        Self::mm(centimeters * 10)
    }

    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub fn cm_f(centimeters: f32) -> Self {
        Lp((centimeters * ARBITRARY_SCALE_F32 * 96. / 2.54).cast())
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub const fn mm(millimeters: i32) -> Self {
        Self(millimeters * ARBITRARY_SCALE_I32 * 960 / 254)
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub fn mm_f(millimeters: f32) -> Self {
        Lp((millimeters * ARBITRARY_SCALE_F32 * 96. / 25.4).cast())
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub const fn inches(inches: i32) -> Self {
        Self(inches * ARBITRARY_SCALE_I32 * 96)
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub fn inches_f(inches: f32) -> Self {
        Self((inches * ARBITRARY_SCALE_F32 * 96.).cast())
    }
}

impl Pow for Lp {
    fn pow(&self, exp: u32) -> Self {
        Self(self.0.saturating_pow(exp))
    }
}

impl Abs for Lp {
    fn abs(&self) -> Self {
        Self(self.0.saturating_abs())
    }
}

impl IntoSigned for Lp {
    type Signed = Self;

    fn into_signed(self) -> Self::Signed {
        self
    }
}

impl fmt::Debug for Lp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fractional = self.0 % ARBITRARY_SCALE_I32;
        let whole = self.0 / ARBITRARY_SCALE_I32;
        if fractional == 0 {
            write!(f, "{whole}lp")
        } else {
            let as_float =
                f64::from(whole) + f64::from(fractional) / f64::from(ARBITRARY_SCALE_F32);
            write!(f, "{as_float}lp")
        }
    }
}

impl fmt::Display for Lp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

define_integer_type!(Px, i32, "docs/px.md", 4);

impl Pow for Px {
    fn pow(&self, exp: u32) -> Self {
        Self(self.0.saturating_pow(exp) / 4_i32.pow(exp.saturating_sub(1)))
    }
}

impl Abs for Px {
    fn abs(&self) -> Self {
        Self(self.0.saturating_abs())
    }
}

impl IntoUnsigned for Px {
    type Unsigned = UPx;

    fn into_unsigned(self) -> Self::Unsigned {
        UPx(self.0.into_unsigned())
    }
}

impl IntoSigned for Px {
    type Signed = Self;

    fn into_signed(self) -> Self::Signed {
        self
    }
}

impl ScreenScale for Px {
    type Lp = Lp;
    type Px = Self;
    type UPx = UPx;

    fn into_px(self, _scale: Fraction) -> Self::Px {
        self
    }

    fn from_px(px: Self::Px, _scale: Fraction) -> Self {
        px
    }

    fn into_lp(self, scale: Fraction) -> Self::Lp {
        Lp(self.0 * ARBITRARY_SCALE_I32 / scale / 4)
    }

    fn from_lp(lp: Self::Lp, scale: Fraction) -> Self {
        lp.into_px(scale)
    }

    fn into_upx(self, _scale: crate::Fraction) -> Self::UPx {
        self.into_unsigned()
    }

    fn from_upx(px: Self::UPx, _scale: crate::Fraction) -> Self {
        px.into_signed()
    }
}

impl IntoComponents<Px> for i32 {
    fn into_components(self) -> (Px, Px) {
        (Px::new(self), Px::new(self))
    }
}

impl IntoComponents<Px> for f32 {
    fn into_components(self) -> (Px, Px) {
        let value = Px::from_float(self);
        (value, value)
    }
}

impl fmt::Debug for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0 >> 2;
        let remainder = self.0 & 0b11;
        match remainder {
            1 => write!(f, "{whole}.25px",),
            2 => write!(f, "{whole}.5px",),
            3 => write!(f, "{whole}.75px",),
            _ => write!(f, "{whole}px",),
        }
    }
}

impl fmt::Display for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::ops::Neg for Px {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl TryFrom<u32> for Px {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value.try_into().map(Self::new)
    }
}

impl PartialEq<UPx> for Px {
    fn eq(&self, other: &UPx) -> bool {
        if let Ok(unsigned) = UPx::try_from(*self) {
            unsigned == *other
        } else {
            false
        }
    }
}

impl PartialOrd<UPx> for Px {
    fn partial_cmp(&self, other: &UPx) -> Option<Ordering> {
        if let Ok(unsigned) = UPx::try_from(*self) {
            Some(unsigned.cmp(other))
        } else {
            Some(Ordering::Less)
        }
    }
}

define_integer_type!(UPx, u32, "docs/upx.md", 4);

impl Pow for UPx {
    fn pow(&self, exp: u32) -> Self {
        Self(self.0.saturating_pow(exp) / 4_u32.pow(exp.saturating_sub(1)))
    }
}

impl IntoSigned for UPx {
    type Signed = Px;

    fn into_signed(self) -> Self::Signed {
        Px(self.0.into_signed())
    }
}

impl IntoUnsigned for UPx {
    type Unsigned = Self;

    fn into_unsigned(self) -> Self::Unsigned {
        self
    }
}

impl ScreenScale for UPx {
    type Lp = Lp;
    type Px = Px;
    type UPx = Self;

    fn into_px(self, _scale: Fraction) -> Self::Px {
        Px(i32::try_from(self.0).unwrap_or(i32::MAX))
    }

    fn from_px(px: Self::Px, _scale: Fraction) -> Self {
        Self::try_from(px).unwrap_or(Self::MIN)
    }

    fn into_lp(self, scale: Fraction) -> Self::Lp {
        (self.0 * ARBITRARY_SCALE_U32 / scale / 4)
            .try_into()
            .unwrap_or(Lp::MAX)
    }

    fn from_lp(lp: Self::Lp, scale: Fraction) -> Self {
        lp.into_px(scale).try_into().unwrap_or(Self::MIN)
    }

    fn into_upx(self, _scale: crate::Fraction) -> Self::UPx {
        self
    }

    fn from_upx(px: Self::UPx, _scale: crate::Fraction) -> Self {
        px
    }
}

impl IntoComponents<UPx> for u32 {
    fn into_components(self) -> (UPx, UPx) {
        (UPx::new(self), UPx::new(self))
    }
}

impl IntoComponents<UPx> for f32 {
    fn into_components(self) -> (UPx, UPx) {
        let value = UPx::from_float(self);
        (value, value)
    }
}

impl fmt::Debug for UPx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0 / 4;
        let remainder = self.0 % 4;
        match remainder {
            1 => write!(f, "{whole}.25px",),
            2 => write!(f, "{whole}.5px",),
            3 => write!(f, "{whole}.75px",),
            _ => write!(f, "{whole}px",),
        }
    }
}

impl fmt::Display for UPx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl TryFrom<UPx> for i32 {
    type Error = TryFromIntError;

    fn try_from(value: UPx) -> Result<Self, Self::Error> {
        value.get().try_into()
    }
}

impl TryFrom<i32> for UPx {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        value.try_into().map(Self::new)
    }
}

impl TryFrom<Px> for UPx {
    type Error = TryFromIntError;

    fn try_from(value: Px) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl TryFrom<UPx> for Px {
    type Error = TryFromIntError;

    fn try_from(value: UPx) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl PartialEq<Px> for UPx {
    fn eq(&self, other: &Px) -> bool {
        if let Ok(unsigned) = UPx::try_from(*other) {
            unsigned == *self
        } else {
            false
        }
    }
}

impl PartialOrd<Px> for UPx {
    fn partial_cmp(&self, other: &Px) -> Option<Ordering> {
        if let Ok(unsigned) = UPx::try_from(*other) {
            Some(unsigned.cmp(self))
        } else {
            Some(Ordering::Less)
        }
    }
}
