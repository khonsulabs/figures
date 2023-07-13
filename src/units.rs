use std::cmp::Ordering;
use std::fmt;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::traits::{
    FloatConversion, IntoComponents, IntoSigned, IntoUnsigned, IsZero, ScreenScale,
};
use crate::utils::lossy_f32_to_i32;
use crate::Fraction;

const ARBITRARY_SCALE: i32 = 182_880;
const ARBITRARY_SCALE_U32: u32 = ARBITRARY_SCALE.unsigned_abs();
#[allow(clippy::cast_precision_loss)]
const ARBITRARY_SCALE_F32: f32 = ARBITRARY_SCALE as f32;

macro_rules! define_integer_type {
    ($name:ident, $inner:ty, $docs_file:literal) => {
        #[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
        #[doc = include_str!($docs_file)]
        #[repr(C)]
        pub struct $name(pub $inner);

        impl $name {
            /// The maximum value for this type.
            pub const MAX: Self = Self(<$inner>::MAX);
            /// The minimum value for this type.
            pub const MIN: Self = Self(<$inner>::MIN);
            /// Zero for this type.
            pub const ZERO: Self = Self(0);
        }

        impl From<$name> for f32 {
            fn from(value: $name) -> Self {
                value.into_float()
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl PartialEq<$inner> for $name {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        impl PartialOrd<$inner> for $name {
            fn partial_cmp(&self, other: &$inner) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
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
                Self((self.0 as f32 / rhs).round() as $inner)
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
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
                Self((self.0 as f32 % rhs).round() as $inner)
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                self * rhs.0
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
                Self((self.0 as f32 * rhs).round() as $inner)
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
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
                Self(self.0 + rhs)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl AddAssign<$inner> for $name {
            fn add_assign(&mut self, rhs: $inner) {
                self.0 += rhs;
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self - rhs.0
            }
        }

        impl Sub<$inner> for $name {
            type Output = Self;

            fn sub(self, rhs: $inner) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl SubAssign<$inner> for $name {
            fn sub_assign(&mut self, rhs: $inner) {
                self.0 -= rhs;
            }
        }

        impl IsZero for $name {
            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }

        impl From<f32> for $name {
            fn from(value: f32) -> Self {
                Self(value as $inner)
            }
        }

        impl FloatConversion for $name {
            type Float = f32;

            #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
            fn into_float(self) -> Self::Float {
                self.0 as f32
            }

            fn from_float(float: Self::Float) -> Self {
                Self::from(float)
            }
        }
    };
}

define_integer_type!(Lp, i32, "docs/lp.md");

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

    fn into_px(self, scale: Fraction) -> Self::Px {
        Px(self.0 * 96 / scale / ARBITRARY_SCALE)
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
        Self(points * ARBITRARY_SCALE / 72)
    }

    /// Returns a value equivalent to the number of `points` provided. One
    /// [point](https://en.wikipedia.org/wiki/Point_(typography)) is 1/72 of an
    /// inch.
    #[must_use]
    pub fn points_f(points: f32) -> Self {
        Lp(lossy_f32_to_i32(points * ARBITRARY_SCALE_F32 / 72.))
    }

    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub const fn cm(centimeters: i32) -> Self {
        Self::mm(centimeters * 10)
    }

    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub fn cm_f(centimeters: f32) -> Self {
        Lp(lossy_f32_to_i32(centimeters * ARBITRARY_SCALE_F32 / 2.54))
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub const fn mm(millimeters: i32) -> Self {
        Self(millimeters * ARBITRARY_SCALE * 10 / 254)
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub fn mm_f(millimeters: f32) -> Self {
        Lp(lossy_f32_to_i32(millimeters * ARBITRARY_SCALE_F32 / 25.4))
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub const fn inches(inches: i32) -> Self {
        Self(inches * ARBITRARY_SCALE)
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub fn inches_f(inches: f32) -> Self {
        Self(lossy_f32_to_i32(inches * ARBITRARY_SCALE_F32))
    }

    /// Raises this value to power of `exp`.
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    /// Returns the square root of this value.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn sqrt(self) -> Self {
        Self(f64::from(self.0).sqrt() as i32)
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
        write!(f, "{}lp", self.0)
    }
}
define_integer_type!(Px, i32, "docs/px.md");

impl Px {
    /// Raises this value to power of `exp`.
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    /// Returns the square root of this value.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn sqrt(self) -> Self {
        Self(f64::from(self.0).sqrt() as i32)
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

    fn into_px(self, _scale: Fraction) -> Self::Px {
        self
    }

    fn from_px(px: Self::Px, _scale: Fraction) -> Self {
        px
    }

    fn into_lp(self, scale: Fraction) -> Self::Lp {
        Lp(self.0 * ARBITRARY_SCALE * scale / 96)
    }

    fn from_lp(lp: Self::Lp, scale: Fraction) -> Self {
        lp.into_px(scale)
    }
}

impl IntoComponents<Px> for i32 {
    fn into_components(self) -> (Px, Px) {
        (Px(self), Px(self))
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
        write!(f, "{}px", self.0)
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
        value.try_into().map(Self)
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

define_integer_type!(UPx, u32, "docs/upx.md");

impl UPx {
    /// Raises this value to power of `exp`.
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    /// Returns the square root of this value.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn sqrt(self) -> Self {
        Self(f64::from(self.0).sqrt() as u32)
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

    fn into_px(self, _scale: Fraction) -> Self::Px {
        Px::try_from(self.0).unwrap_or(Px::MAX)
    }

    fn from_px(px: Self::Px, _scale: Fraction) -> Self {
        Self::try_from(px).unwrap_or(Self::MIN)
    }

    fn into_lp(self, scale: Fraction) -> Self::Lp {
        (self.0 * ARBITRARY_SCALE_U32 * scale / 96)
            .try_into()
            .unwrap_or(Lp::MAX)
    }

    fn from_lp(lp: Self::Lp, scale: Fraction) -> Self {
        (lp.0 * ARBITRARY_SCALE * scale / 96)
            .try_into()
            .unwrap_or(Self::MIN)
    }
}

impl IntoComponents<UPx> for u32 {
    fn into_components(self) -> (UPx, UPx) {
        (UPx(self), UPx(self))
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
        write!(f, "{}px", self.0)
    }
}

impl TryFrom<UPx> for i32 {
    type Error = TryFromIntError;

    fn try_from(value: UPx) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<i32> for UPx {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
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
        value.0.try_into()
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
