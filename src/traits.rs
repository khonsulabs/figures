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

/// Converts this type into its measurement in [`Px`](crate::units::Px).
pub trait IntoPixels {
    /// This type when measuring with [`Px`](crate::units::Px).
    type Px;

    /// Converts this value from its current unit into device pixels ([`Px`](crate::units::Px))
    /// using the provided `scale` factor.
    fn into_px(self, scale: Fraction) -> Self::Px;
}

/// Converts this type into its measurement in [`Dips`](crate::units::Dips).
pub trait IntoDips {
    /// This type when measuring with [`Dips`](crate::units::Dips).
    type Dips;

    /// Converts this value from its current unit into device independent pixels
    /// ([`Dips`](crate::units::Dips)) using the provided `scale` factor.
    fn into_dips(self, scale: Fraction) -> Self::Dips;
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
