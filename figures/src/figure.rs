use std::{
    any::type_name,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use num_traits::{NumCast, One, ToPrimitive, Zero};

use crate::{DisplayScale, Pixels, Points, Scale, Scaled, ToPixels, ToPoints, ToScaled};

/// A value in a specific unit.
pub struct Figure<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T: Debug, Unit> Debug for Figure<T, Unit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!(
            "Figure<{}, {}>",
            type_name::<T>(),
            type_name::<Unit>()
        ))
        .field(&self.value)
        .finish()
    }
}

impl<T: Copy, Unit> Copy for Figure<T, Unit> {}
impl<T: Clone, Unit> Clone for Figure<T, Unit> {
    fn clone(&self) -> Self {
        Self::from(self.value.clone())
    }
}

impl<T: Copy, Unit> Figure<T, Unit> {
    /// Returns the inner value
    pub fn get(&self) -> T {
        self.value
    }
}

impl<T, Unit> From<T> for Figure<T, Unit> {
    fn from(value: T) -> Self {
        Self {
            value,
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> ToPrimitive for Figure<T, Unit>
where
    T: ToPrimitive,
{
    fn to_i64(&self) -> Option<i64> {
        T::to_i64(&self.value)
    }

    fn to_u64(&self) -> Option<u64> {
        T::to_u64(&self.value)
    }

    fn to_isize(&self) -> Option<isize> {
        T::to_isize(&self.value)
    }

    fn to_i8(&self) -> Option<i8> {
        T::to_i8(&self.value)
    }

    fn to_i16(&self) -> Option<i16> {
        T::to_i16(&self.value)
    }

    fn to_i32(&self) -> Option<i32> {
        T::to_i32(&self.value)
    }

    fn to_i128(&self) -> Option<i128> {
        T::to_i128(&self.value)
    }

    fn to_usize(&self) -> Option<usize> {
        T::to_usize(&self.value)
    }

    fn to_u8(&self) -> Option<u8> {
        T::to_u8(&self.value)
    }

    fn to_u16(&self) -> Option<u16> {
        T::to_u16(&self.value)
    }

    fn to_u32(&self) -> Option<u32> {
        T::to_u32(&self.value)
    }

    fn to_u128(&self) -> Option<u128> {
        T::to_u128(&self.value)
    }

    fn to_f32(&self) -> Option<f32> {
        T::to_f32(&self.value)
    }

    fn to_f64(&self) -> Option<f64> {
        T::to_f64(&self.value)
    }
}

impl<T, Unit> NumCast for Figure<T, Unit>
where
    T: NumCast,
{
    fn from<N: num_traits::ToPrimitive>(n: N) -> Option<Self> {
        T::from(n).map(<Self as std::convert::From<T>>::from)
    }
}

impl<T, Unit> Default for Figure<T, Unit>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Eq for Figure<T, Unit> where T: Eq {}

impl<T, Unit> PartialEq for Figure<T, Unit>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T, Unit> Ord for Figure<T, Unit>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T, Unit> PartialOrd for Figure<T, Unit>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, Unit> Add for Figure<T, Unit>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.add(rhs.value),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Sub for Figure<T, Unit>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.sub(rhs.value),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Mul for Figure<T, Unit>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.mul(rhs.value),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Div for Figure<T, Unit>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.div(rhs.value),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Rem for Figure<T, Unit>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.rem(rhs.value),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Neg for Figure<T, Unit>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: self.value.neg(),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> AddAssign for Figure<T, Unit>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value.add_assign(rhs.value);
    }
}

impl<T, Unit> SubAssign for Figure<T, Unit>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value.sub_assign(rhs.value);
    }
}

impl<T, Unit> DivAssign for Figure<T, Unit>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.value.div_assign(rhs.value);
    }
}

impl<T, Unit> MulAssign for Figure<T, Unit>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.value.mul_assign(rhs.value);
    }
}

impl<T, Unit> RemAssign for Figure<T, Unit>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.value.rem_assign(rhs.value);
    }
}

impl<T, Unit> One for Figure<T, Unit>
where
    T: One,
{
    fn one() -> Self {
        Self::from(T::one())
    }
}

impl<T, Unit> Zero for Figure<T, Unit>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::from(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<T, UnitA, UnitB> Mul<Scale<T, UnitA, UnitB>> for Figure<T, UnitA>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Figure<T, UnitB>;

    fn mul(self, rhs: Scale<T, UnitA, UnitB>) -> Self::Output {
        Figure {
            value: self.value.mul(rhs.get()),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, UnitA, UnitB> Div<Scale<T, UnitA, UnitB>> for Figure<T, UnitB>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Figure<T, UnitA>;

    fn div(self, rhs: Scale<T, UnitA, UnitB>) -> Self::Output {
        Figure {
            value: self.value.div(rhs.get()),
            _unit: PhantomData::default(),
        }
    }
}

impl<T> ToScaled<T> for Figure<T, Scaled>
where
    T: Copy,
{
    type Scaled = Self;

    fn to_scaled(&self, _scale: &DisplayScale<T>) -> Self::Scaled {
        *self
    }
}

impl<T> ToScaled<T> for Figure<T, Points>
where
    T: Mul<T, Output = T> + Copy,
{
    type Scaled = Figure<T, Scaled>;

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.between
    }
}

impl<T> ToScaled<T> for Figure<T, Pixels>
where
    T: Mul<T, Output = T> + Copy,
{
    type Scaled = Figure<T, Scaled>;

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.scaled
    }
}

impl<T> ToPoints<T> for Figure<T, Scaled>
where
    T: Div<T, Output = T> + Copy,
{
    type Points = Figure<T, Points>;

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self / scale.between
    }
}

impl<T> ToPoints<T> for Figure<T, Points>
where
    T: Copy,
{
    type Points = Self;

    fn to_points(&self, _scale: &DisplayScale<T>) -> Self::Points {
        *self
    }
}

impl<T> ToPoints<T> for Figure<T, Pixels>
where
    T: Mul<T, Output = T> + Copy,
{
    type Points = Figure<T, Points>;

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self * scale.points
    }
}

impl<T> ToPixels<T> for Figure<T, Scaled>
where
    T: Div<T, Output = T> + Copy,
{
    type Pixels = Figure<T, Pixels>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.scaled
    }
}

impl<T> ToPixels<T> for Figure<T, Points>
where
    T: Div<T, Output = T> + Copy,
{
    type Pixels = Figure<T, Pixels>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.points
    }
}

impl<T> ToPixels<T> for Figure<T, Pixels>
where
    T: Copy,
{
    type Pixels = Self;

    fn to_pixels(&self, _scale: &DisplayScale<T>) -> Self::Pixels {
        *self
    }
}
