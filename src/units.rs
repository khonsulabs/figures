use std::fmt;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::traits::{FloatConversion, IntoPixels, Zero};
use crate::utils::lossy_f32_to_i32;
use crate::Ratio;

macro_rules! define_integer_type {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
        #[repr(C)]
        pub struct $name(pub $inner);

        impl $name {
            #[must_use]
            pub const fn div(self, rhs: $inner) -> Self {
                Self(self.0 / rhs)
            }
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
            type Output = $inner;

            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Div<$inner> for $name {
            type Output = Self;

            fn div(self, rhs: $inner) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl DivAssign<$inner> for $name {
            fn div_assign(&mut self, rhs: $inner) {
                self.0 /= rhs;
            }
        }

        impl Div<f32> for $name {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self((self.0 as f32 / rhs).round() as $inner)
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

        impl SubAssign<$inner> for $name {
            fn sub_assign(&mut self, rhs: $inner) {
                self.0 -= rhs;
            }
        }

        impl Zero for $name {
            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }
    };
}

define_integer_type!(Dip, i32);
define_integer_type!(Px, i32);
define_integer_type!(UPx, u32);

impl IntoPixels for Dip {
    type Px = Px;

    fn into_pixels(self, scale: Ratio) -> Self::Px {
        Px(self.0 * scale * 96 / 2540)
    }
}

impl std::ops::Neg for Dip {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Neg for Px {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl TryFrom<UPx> for Px {
    type Error = TryFromIntError;

    fn try_from(value: UPx) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl TryFrom<Px> for UPx {
    type Error = TryFromIntError;

    fn try_from(value: Px) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl Dip {
    pub const CM: Self = Dip(1000);
    pub const INCH: Self = Dip(2540);
    pub const MM: Self = Self::CM.div(10);

    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn sqrt(self) -> Self {
        Self(f64::from(self.0).sqrt() as i32)
    }
}

impl From<f32> for Dip {
    fn from(cm: f32) -> Self {
        Dip(lossy_f32_to_i32(cm * 1000.))
    }
}

impl FloatConversion for Dip {
    type Float = f32;

    #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
    fn into_float(self) -> Self::Float {
        self.0 as f32 / 1000.
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for Dip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}dip", self.0)
    }
}

impl Px {
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn sqrt(self) -> Self {
        Self(f64::from(self.0).sqrt() as i32)
    }
}

impl From<f32> for Px {
    #[allow(clippy::cast_possible_truncation)] // truncation desired
    fn from(pixels: f32) -> Self {
        Px(pixels as i32)
    }
}

impl FloatConversion for Px {
    type Float = f32;

    #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
    fn into_float(self) -> Self::Float {
        self.0 as f32
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

impl From<f32> for UPx {
    #[allow(clippy::cast_possible_truncation)] // truncation desired
    #[allow(clippy::cast_sign_loss)] // sign loss is handled
    fn from(pixels: f32) -> Self {
        if pixels < 0. {
            Self(0)
        } else {
            Self(pixels as u32)
        }
    }
}

impl FloatConversion for UPx {
    type Float = f32;

    #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
    fn into_float(self) -> Self::Float {
        self.0 as f32
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for UPx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}
