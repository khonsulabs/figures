use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::time::Duration;

use intentional::{Cast, CastInto};

use crate::units::{Lp, Px, UPx, ARBITRARY_SCALE};
use crate::Fraction;

/// Converts a type to its floating point representation.
///
/// This trait exists because there is no trait in Rust to perform `x as f32`.
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

/// A type that can represent a zero-value.
pub trait Zero {
    /// The zero value for this type.
    const ZERO: Self;

    /// Returns true if `self` represents `0`.
    fn is_zero(&self) -> bool;
}

macro_rules! impl_int_zero {
    ($type:ident) => {
        impl Zero for $type {
            const ZERO: Self = 0;

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

/// A type that can have its absolute difference from zero calculated.
pub trait Abs {
    /// Returns the positive difference between this value and 0.
    ///
    /// This function should never panic and always perform a saturating
    /// absolute value calculation.
    #[must_use]
    fn abs(&self) -> Self;
}

macro_rules! impl_int_abs {
    ($type:ident) => {
        impl Abs for $type {
            fn abs(&self) -> Self {
                self.saturating_abs()
            }
        }
    };
}

impl_int_abs!(i8);
impl_int_abs!(i16);
impl_int_abs!(i32);
impl_int_abs!(i64);
impl_int_abs!(i128);
impl_int_abs!(isize);

impl Abs for f32 {
    fn abs(&self) -> Self {
        (*self).abs()
    }
}

/// Raises a value to an exponent.
pub trait Pow {
    /// Returns the saturating result of raising `self` to the `exp` power.
    #[must_use]
    fn pow(&self, exp: u32) -> Self;
}

macro_rules! impl_int_pow {
    ($type:ident) => {
        impl Pow for $type {
            fn pow(&self, exp: u32) -> Self {
                self.saturating_pow(exp)
            }
        }
    };
}

impl_int_pow!(i8);
impl_int_pow!(i16);
impl_int_pow!(i32);
impl_int_pow!(i64);
impl_int_pow!(i128);
impl_int_pow!(isize);
impl_int_pow!(u8);
impl_int_pow!(u16);
impl_int_pow!(u32);
impl_int_pow!(u64);
impl_int_pow!(u128);
impl_int_pow!(usize);

impl Pow for f32 {
    fn pow(&self, exp: u32) -> Self {
        self.powf(exp.cast())
    }
}

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

/// Constructors for types that are composed of two [`Px`] components.
pub trait Px2D: FromComponents<Px> {
    /// Returns a new value containing the x and y components converted into
    /// [`Px`].
    fn px(x: impl Into<Px>, y: impl Into<Px>) -> Self {
        Self::from_components((x.into(), y.into()))
    }
}

impl<T> Px2D for T where T: FromComponents<Px> {}

/// Constructors for types that are composed of two [`UPx`] components.
pub trait UPx2D: FromComponents<UPx> {
    /// Returns a new value containing the x and y components converted into
    /// [`UPx`].
    fn upx(x: impl Into<UPx>, y: impl Into<UPx>) -> Self {
        Self::from_components((x.into(), y.into()))
    }
}

impl<T> UPx2D for T where T: FromComponents<UPx> {}

/// Constructors for types that are composed of two [`Lp`] components.
pub trait Lp2D: FromComponents<Lp> {
    /// Returns a new value containing the x and y components converted into
    /// [`Lp`] using [`Lp::points`]/[`Lp::points_f`].
    fn points(x: impl Into<FloatOrInt>, y: impl Into<FloatOrInt>) -> Self {
        Self::from_components((x.into().into_points(), y.into().into_points()))
    }

    /// Returns a new value containing the x and y components converted into
    /// [`Lp`] using [`Lp::cm`]/[`Lp::cm_f`].
    fn cm(x: impl Into<FloatOrInt>, y: impl Into<FloatOrInt>) -> Self {
        Self::from_components((x.into().into_cm(), y.into().into_cm()))
    }

    /// Returns a new value containing the x and y components converted into
    /// [`Lp`] using [`Lp::mm`]/[`Lp::mm_f`].
    fn mm(x: impl Into<FloatOrInt>, y: impl Into<FloatOrInt>) -> Self {
        Self::from_components((x.into().into_mm(), y.into().into_mm()))
    }

    /// Returns a new value containing the x and y components converted into
    /// [`Lp`] using [`Lp::inches`]/[`Lp::inches_f`].
    fn inches(x: impl Into<FloatOrInt>, y: impl Into<FloatOrInt>) -> Self {
        Self::from_components((x.into().into_inches(), y.into().into_inches()))
    }
}

impl<T> Lp2D for T where T: FromComponents<Lp> {}

/// A type representing either an [`i32`] or an [`f32`].
#[derive(Clone, Copy)]
pub enum FloatOrInt {
    /// An integer value.
    Int(i32),
    /// A floating point value.
    Float(f32),
}

impl FloatOrInt {
    fn map<R>(self, float: impl FnOnce(f32) -> R, int: impl FnOnce(i32) -> R) -> R {
        match self {
            FloatOrInt::Int(value) => int(value),
            FloatOrInt::Float(value) => float(value),
        }
    }

    /// Returns this number as [`Lp`] using [`Lp::points`]/[`Lp::points_f`].
    pub fn into_points(self) -> Lp {
        self.map(Lp::points_f, Lp::points)
    }

    /// Returns this number as [`Lp`] using [`Lp::cm`]/[`Lp::cm_f`].
    #[must_use]
    pub fn into_cm(self) -> Lp {
        self.map(Lp::cm_f, Lp::cm)
    }

    /// Returns this number as [`Lp`] using [`Lp::mm`]/[`Lp::mm_f`].
    #[must_use]
    pub fn into_mm(self) -> Lp {
        self.map(Lp::mm_f, Lp::mm)
    }

    /// Returns this number as [`Lp`] using [`Lp::inches`]/[`Lp::inches_f`].
    #[must_use]
    pub fn into_inches(self) -> Lp {
        self.map(Lp::inches_f, Lp::inches)
    }
}

impl From<i32> for FloatOrInt {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for FloatOrInt {
    fn from(value: f32) -> Self {
        Self::Float(value)
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
    /// This type when measuring with [`UPx`](crate::units::UPx).
    type UPx;
    /// This type when measuring with [`Lp`](crate::units::Lp).
    type Lp;

    /// Converts this value from its current unit into device pixels ([`Px`](crate::units::Px))
    /// using the provided `scale` factor.
    fn into_px(self, scale: Fraction) -> Self::Px;
    /// Converts from pixels into this type, using the provided `scale` factor.
    fn from_px(px: Self::Px, scale: Fraction) -> Self;

    /// Converts this value from its current unit into device pixels
    /// ([`UPx`](crate::units::UPx)) using the provided `scale` factor.
    fn into_upx(self, scale: Fraction) -> Self::UPx;
    /// Converts from unsigned pixels into this type, using the provided `scale` factor.
    fn from_upx(px: Self::UPx, scale: Fraction) -> Self;

    /// Converts this value from its current unit into device independent pixels
    /// ([`Lp`](crate::units::Lp)) using the provided `scale` factor.
    fn into_lp(self, scale: Fraction) -> Self::Lp;
    /// Converts from [`Lp`](crate::units::Lp) into this type, using the provided `scale` factor.
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

impl IntoSigned for f32 {
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
    + Zero
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

/// Common number operations available on number types in Rust that aren't
/// available as traits.
pub trait StdNumOps {
    /// Adds `self` and `other`, saturating instead of overflowing.
    #[must_use]
    fn saturating_add(self, other: Self) -> Self;
    /// Multiplies `self` and `other`, saturating instead of overflowing.
    #[must_use]
    fn saturating_mul(self, other: Self) -> Self;
    /// Divides `self` by `other`, saturating instead of overflowing.
    #[must_use]
    fn saturating_div(self, other: Self) -> Self;
    /// Subtracts `other` from `self`, saturating instead of overflowing.
    #[must_use]
    fn saturating_sub(self, other: Self) -> Self;
}

macro_rules! impl_std_num_ops {
    ($type:ident) => {
        impl StdNumOps for $type {
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

impl_std_num_ops!(u8);

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
        + Zero
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
pub trait ScreenUnit: UnscaledUnit + ScreenScale<Px = Px, Lp = Lp, UPx = UPx> + Unit {}

impl<T> ScreenUnit for T where T: UnscaledUnit + ScreenScale<Px = Px, Lp = Lp, UPx = UPx> + Unit {}

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

impl Ranged for Duration {
    const MAX: Self = Duration::MAX;
    const MIN: Self = Duration::ZERO;
}

impl Ranged for bool {
    const MAX: Self = true;
    const MIN: Self = false;
}

/// A type that has a scaling factor when converting to pixels.
pub trait PixelScaling {
    /// The scaling factor to apply when converting to pixels, in addition to
    /// any spatial scaling already being applied.
    const PX_SCALING_FACTOR: u16;
}

impl PixelScaling for Px {
    const PX_SCALING_FACTOR: u16 = 1;
}

impl PixelScaling for UPx {
    const PX_SCALING_FACTOR: u16 = 1;
}

impl PixelScaling for Lp {
    const PX_SCALING_FACTOR: u16 = ARBITRARY_SCALE; // ARBITRARY_SCALE / 96
}

/// Information about scaling for a numerical unit type.
pub trait UnscaledUnit {
    /// The internal representation used by this type.
    type Representation: CastInto<i32>;

    /// Returns a new instance using the unscaled representation.
    fn from_unscaled(unscaled: Self::Representation) -> Self;
    /// Returns the inner, unscaled representation of this value.
    fn into_unscaled(self) -> Self::Representation;
}

/// Functionality for rounding values to whole numbers.
pub trait Round {
    /// Returns `self` rounded to the nearest whole number.
    #[must_use]
    fn round(self) -> Self;
    /// Returns `self` raised to the next whole number further away from 0.
    #[must_use]
    fn ceil(self) -> Self;
    /// Returns `self` lowered to the next whole number closer to 0.
    #[must_use]
    fn floor(self) -> Self;
}

impl Round for f32 {
    fn round(self) -> Self {
        self.round()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn floor(self) -> Self {
        self.floor()
    }
}

/// Functionality for getting the root of a number.
pub trait Roots {
    /// Returns the square root of `self`.
    #[must_use]
    fn sqrt(self) -> Self;

    /// Returns the cube root of `self`.
    #[must_use]
    fn cbrt(self) -> Self;
}

impl Roots for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
