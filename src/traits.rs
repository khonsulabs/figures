use crate::Ratio;

pub trait FloatConversion {
    type Float;
    fn into_float(self) -> Self::Float;
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

pub trait Zero {
    fn is_zero(&self) -> bool;
}

impl Zero for i32 {
    fn is_zero(&self) -> bool {
        *self != 0
    }
}

impl Zero for u32 {
    fn is_zero(&self) -> bool {
        *self != 0
    }
}

pub trait FromComponents<Unit>: Sized {
    fn from_components(components: (Unit, Unit)) -> Self;
}

pub trait IntoComponents<Unit>: Sized {
    fn into_components(self) -> (Unit, Unit);

    fn to<Type>(self) -> Type
    where
        Type: FromComponents<Unit>,
    {
        Type::from_components(self.into_components())
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

pub trait IntoPixels {
    type Px;

    fn into_pixels(self, scale: Ratio) -> Self::Px;
}
