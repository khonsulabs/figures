use std::fmt;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::traits::{
    FloatConversion, IntoComponents, IntoDips, IntoPixels, IntoSigned, IntoUnsigned, IsZero,
};
use crate::utils::lossy_f32_to_i32;
use crate::Fraction;

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

define_integer_type!(Dips, i32, "docs/dips.md");

impl IntoComponents<Dips> for i32 {
    fn into_components(self) -> (Dips, Dips) {
        (Dips(self), Dips(self))
    }
}

impl IntoComponents<Dips> for f32 {
    fn into_components(self) -> (Dips, Dips) {
        let value = Dips::from_float(self);
        (value, value)
    }
}

impl IntoPixels for Dips {
    type Px = Px;

    fn into_px(self, scale: Fraction) -> Self::Px {
        Px(self.0 * scale * 96 / 2540)
    }
}

impl std::ops::Neg for Dips {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl TryFrom<u32> for Dips {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl Dips {
    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub const fn cm(centimeters: i32) -> Self {
        Self::mm(centimeters * 10)
    }

    /// Returns a value equivalent to the number of `centimeters` provided.
    #[must_use]
    pub fn cm_f(centimeters: f32) -> Self {
        Dips(lossy_f32_to_i32(centimeters * 1000.))
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub const fn mm(millimeters: i32) -> Self {
        Self(100 * millimeters)
    }

    /// Returns a value equivalent to the number of `millimeters` provided.
    #[must_use]
    pub fn mm_f(millimeters: f32) -> Self {
        Dips(lossy_f32_to_i32(millimeters * 100.))
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub const fn inches(inches: i32) -> Self {
        Self(inches * 2540)
    }

    /// Returns a value equivalent to the number of `inches` provided.
    #[must_use]
    pub fn inches_f(inches: f32) -> Self {
        Dips(lossy_f32_to_i32(inches * 2540.))
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

impl IntoSigned for Dips {
    type Signed = Self;

    fn into_signed(self) -> Self::Signed {
        self
    }
}

impl fmt::Debug for Dips {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}dip", self.0)
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

impl IntoDips for Px {
    type Dips = Dips;

    fn into_dips(self, scale: Fraction) -> Self::Dips {
        Dips(self.0 / scale * 2540 / 96)
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

impl IntoPixels for UPx {
    type Px = Px;

    fn into_px(self, _scale: Fraction) -> Self::Px {
        Px::try_from(self.0).unwrap_or(Px::MAX)
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

impl IntoDips for UPx {
    type Dips = Dips;

    fn into_dips(self, scale: Fraction) -> Self::Dips {
        (self.0 / scale * 2540 / 96).try_into().unwrap_or(Dips::MAX)
    }
}

impl TryFrom<UPx> for Px {
    type Error = TryFromIntError;

    fn try_from(value: UPx) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}
