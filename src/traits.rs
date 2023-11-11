use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::units::{Lp, Px, UPx};
use crate::Fraction;

/// Converts a type to its floating point representation.
///
/// This trait exists because there is no trait in Rust to peform `x as f32`.
pub trait FloatConversion {
    /// The type that represents this type in floating point form.
    type Float;

    /// Returns this value in floating point form.
    fn into_float(self) -> Self::Float;
    /// Converts from floating point to this form.
    fn from_float(float: Self::Float) -> Self;
}

impl FloatConversion for u32 {
    type Float = f32;

    #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
    fn into_float(self) -> Self::Float {
        self as f32
    }

    #[allow(clippy::cast_possible_truncation)] // truncation desired
    #[allow(clippy::cast_sign_loss)] // sign loss is asserted
    fn from_float(float: Self::Float) -> Self {
        assert!(float.is_sign_positive());
        float as u32
    }
}

impl FloatConversion for i32 {
    type Float = f32;

    #[allow(clippy::cast_precision_loss)] // precision loss desired to best approximate the value
    fn into_float(self) -> Self::Float {
        self as f32
    }

    #[allow(clippy::cast_possible_truncation)] // truncation desired
    #[allow(clippy::cast_sign_loss)] // sign loss is asserted
    fn from_float(float: Self::Float) -> Self {
        float as i32
    }
}

/// Allows checking if a type represents a `0`.
pub trait IsZero {
    /// Returns true if `self` represents `0`.
    fn is_zero(&self) -> bool;
}

macro_rules! impl_int_zero {
    ($type:ident) => {
        impl IsZero for $type {
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
    };
}

impl_int_zero!(i8);
impl_int_zero!(i16);
impl_int_zero!(i32);
impl_int_zero!(i64);
impl_int_zero!(i128);
impl_int_zero!(isize);
impl_int_zero!(u8);
impl_int_zero!(u16);
impl_int_zero!(u32);
impl_int_zero!(u64);
impl_int_zero!(u128);
impl_int_zero!(usize);

/// Converts from a 2d vector in tuple form
pub trait FromComponents<Unit>: Sized {
    /// Returns a new instance from the 2d vector components provided.
    fn from_components(components: (Unit, Unit)) -> Self;

    /// Converts this type to another type using [`FromComponents`] and
    /// [`IntoComponents`].
    fn from_vec<Type>(other: Type) -> Self
    where
        Type: IntoComponents<Unit>,
    {
        Self::from_components(other.into_components())
    }
}

/// Converts to a 2d vector in tuple form
pub trait IntoComponents<Unit>: Sized {
    /// Extracts this type's 2d vector components.
    fn into_components(self) -> (Unit, Unit);

    /// Converts this type to another type using [`FromComponents`] and
    /// [`IntoComponents`].
    fn to_vec<Type>(self) -> Type
    where
        Type: FromComponents<Unit>,
    {
        Type::from_vec(self)
    }
}

impl<Unit> FromComponents<Unit> for (Unit, Unit) {
    fn from_components(components: Self) -> Self {
        components
    }
}
impl<Unit> IntoComponents<Unit> for (Unit, Unit) {
    fn into_components(self) -> Self {
        self
    }
}

impl<Unit> IntoComponents<Unit> for Unit
where
    Unit: Copy,
{
    fn into_components(self) -> (Unit, Unit) {
        (self, self)
    }
}

/// Converts this type into its measurement in [`Px`](crate::units::Px) and [`Lp`](crate::units::Lp).
pub trait ScreenScale {
    /// This type when measuring with [`Px`](crate::units::Px).
    type Px;
    /// This type when measuring with [`Lp`](crate::units::Lp).
    type Lp;

    /// Converts this value from its current unit into device pixels ([`Px`](crate::units::Px))
    /// using the provided `scale` factor.
    fn into_px(self, scale: Fraction) -> Self::Px;
    /// Converts from pixels into this type, using the provided `scale` factor.
    fn from_px(px: Self::Px, scale: Fraction) -> Self;

    /// Converts this value from its current unit into device independent pixels
    /// ([`Lp`](crate::units::Lp)) using the provided `scale` factor.
    fn into_lp(self, scale: Fraction) -> Self::Lp;
    /// Converts from Lp into this type, using the provided `scale` factor.
    fn from_lp(lp: Self::Lp, scale: Fraction) -> Self;
}

/// Converts a value into its signed representation, clamping negative numbers
/// to `i32::MAX`.
pub trait IntoSigned {
    /// The signed representation of this type.
    type Signed;
    /// Returns this value as an unsigned value. Values that are larger than can
    /// fit in an `i32` are converted to `i32::MAX`.
    #[must_use]
    fn into_signed(self) -> Self::Signed;
}

impl IntoSigned for u32 {
    type Signed = i32;

    fn into_signed(self) -> Self::Signed {
        self.try_into().unwrap_or(i32::MAX)
    }
}

impl IntoSigned for i32 {
    type Signed = Self;

    fn into_signed(self) -> Self::Signed {
        self
    }
}

/// Converts a value into its signed representation, clamping negative numbers
/// to 0.
pub trait IntoUnsigned {
    /// The unsigned representation of this type.
    type Unsigned;
    /// Returns this value as an unsigned value. Negative values will be
    /// converted to 0.
    #[must_use]
    fn into_unsigned(self) -> Self::Unsigned;
}

impl IntoUnsigned for i32 {
    type Unsigned = u32;

    fn into_unsigned(self) -> Self::Unsigned {
        self.try_into().unwrap_or(0)
    }
}

impl IntoUnsigned for u32 {
    type Unsigned = Self;

    fn into_unsigned(self) -> Self::Unsigned {
        self
    }
}

/// A type that can be used as a `Unit` in figures.
pub trait Unit:
    FloatConversion<Float = f32>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + DivAssign
    + MulAssign
    + RemAssign
    + IsZero
    + Ord
    + Eq
    + Copy
    + Default
    + std::fmt::Debug
    + IntoSigned
    + TryInto<i32>
    + 'static
{
}

impl<T> Unit for T where
    T: FloatConversion<Float = f32>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + DivAssign
        + MulAssign
        + RemAssign
        + IsZero
        + Ord
        + Eq
        + Copy
        + Default
        + std::fmt::Debug
        + IntoSigned
        + TryInto<i32>
        + 'static
{
}

/// A type that can be used as a `Unit` in figures that knows how to convert to
/// [`Lp`] or [`Px`].
pub trait ScreenUnit: ScreenScale<Px = Px, Lp = Lp> + Unit {}

impl<T> ScreenUnit for T where T: ScreenScale<Px = Px, Lp = Lp> + Unit {}

/// A type that has a minimum and a maximum.
pub trait Ranged: Sized {
    /// The minimum value for this type.
    const MIN: Self;
    /// The maximum value for this type.
    const MAX: Self;
}

macro_rules! impl_int_ranged {
    ($type:ident) => {
        impl Ranged for $type {
            const MAX: Self = $type::MAX;
            const MIN: Self = $type::MIN;
        }
    };
}

impl_int_ranged!(i8);
impl_int_ranged!(i16);
impl_int_ranged!(i32);
impl_int_ranged!(i64);
impl_int_ranged!(i128);
impl_int_ranged!(isize);
impl_int_ranged!(u8);
impl_int_ranged!(u16);
impl_int_ranged!(u32);
impl_int_ranged!(u64);
impl_int_ranged!(u128);
impl_int_ranged!(usize);
impl_int_ranged!(f32);
impl_int_ranged!(f64);
impl_int_ranged!(Px);
impl_int_ranged!(UPx);
impl_int_ranged!(Lp);

impl Ranged for bool {
    const MAX: Self = true;
    const MIN: Self = false;
}
