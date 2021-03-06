use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::{NumCast, One, ToPrimitive, Zero};

use crate::{
    num::{Ceil, Floor},
    Approx, DisplayScale, Displayable, Pixels, Points, Round, Scale, Scaled,
};

/// A value in a specific unit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Figure<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T: Debug, Unit> Debug for Figure<T, Unit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Figure").field(&self.value).finish()
    }
}

impl<T: Copy, Unit> Copy for Figure<T, Unit> {}
impl<T: Clone, Unit> Clone for Figure<T, Unit> {
    fn clone(&self) -> Self {
        Self::from(self.value.clone())
    }
}

impl<T, Unit> Figure<T, Unit> {
    /// Returns a new figure with `value`.
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, Unit> Figure<T, Unit> {
    /// Returns the inner value
    pub fn get(&self) -> T {
        self.value
    }

    /// Returns this value with the new unit. Does not affect the underlying
    /// value.
    pub fn cast_unit<NewUnit>(&self) -> Figure<T, NewUnit> {
        Figure::new(self.value)
    }
}

impl<T, Unit> Figure<T, Unit>
where
    T: std::cmp::PartialOrd + Copy,
{
    /// Returns the smaller value of `self` and `rhs`.
    pub fn min(self, rhs: Self) -> Self {
        if self.get() <= rhs.get() {
            self
        } else {
            rhs
        }
    }

    /// Returns the larger value of `self` and `rhs`.
    pub fn max(self, rhs: Self) -> Self {
        if self.get() >= rhs.get() {
            self
        } else {
            rhs
        }
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

impl<T, Unit> Add<T> for Figure<T, Unit>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.add(rhs),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Sub<T> for Figure<T, Unit>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.sub(rhs),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Mul<T> for Figure<T, Unit>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.mul(rhs),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Div<T> for Figure<T, Unit>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.div(rhs),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> Rem<T> for Figure<T, Unit>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            value: self.value.rem(rhs),
            _unit: PhantomData::default(),
        }
    }
}

impl<T, Unit> AddAssign<T> for Figure<T, Unit>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        self.value.add_assign(rhs);
    }
}

impl<T, Unit> SubAssign<T> for Figure<T, Unit>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        self.value.sub_assign(rhs);
    }
}

impl<T, Unit> DivAssign<T> for Figure<T, Unit>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.value.div_assign(rhs);
    }
}

impl<T, Unit> MulAssign<T> for Figure<T, Unit>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.value.mul_assign(rhs);
    }
}

impl<T, Unit> RemAssign<T> for Figure<T, Unit>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: T) {
        self.value.rem_assign(rhs);
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

impl<T> Displayable<T> for Figure<T, Scaled>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Figure<T, Pixels>;
    type Points = Figure<T, Points>;
    type Scaled = Self;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self * scale.total
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self * scale.additional
    }

    fn to_scaled(&self, _scale: &DisplayScale<T>) -> Self::Scaled {
        *self
    }
}

impl<T> Displayable<T> for Figure<T, Points>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Figure<T, Pixels>;
    type Points = Self;
    type Scaled = Figure<T, Scaled>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self * scale.dpi
    }

    fn to_points(&self, _scale: &DisplayScale<T>) -> Self::Points {
        *self
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self / scale.additional
    }
}

impl<T> Displayable<T> for Figure<T, Pixels>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Self;
    type Points = Figure<T, Points>;
    type Scaled = Figure<T, Scaled>;

    fn to_pixels(&self, _scale: &DisplayScale<T>) -> Self::Pixels {
        *self
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self / scale.dpi
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self / scale.total
    }
}

impl<T, Unit> Round for Figure<T, Unit>
where
    T: Round,
{
    fn round(mut self) -> Self {
        self.value = self.value.round();
        self
    }
}

impl<T, Unit> Ceil for Figure<T, Unit>
where
    T: Ceil,
{
    fn ceil(mut self) -> Self {
        self.value = self.value.ceil();
        self
    }
}

impl<T, Unit> Floor for Figure<T, Unit>
where
    T: Floor,
{
    fn floor(mut self) -> Self {
        self.value = self.value.floor();
        self
    }
}

impl<T, Unit> Approx<T> for Figure<T, Unit>
where
    T: AbsDiffEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.value.abs_diff_eq(&other.value, T::default_epsilon())
    }
}

impl<T, Unit> AbsDiffEq for Figure<T, Unit>
where
    T: AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&other.value, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_ne(&other.value, epsilon)
    }
}

impl<T, Unit> UlpsEq for Figure<T, Unit>
where
    T: UlpsEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }

    fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_ne(&other.value, epsilon, max_ulps)
    }
}

impl<T, Unit> RelativeEq for Figure<T, Unit>
where
    T: RelativeEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.value.relative_eq(&other.value, epsilon, max_relative)
    }

    fn relative_ne(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.value.relative_ne(&other.value, epsilon, max_relative)
    }
}

#[test]
fn debug_test() {
    assert_eq!(&format!("{:?}", Figure::<u32, Pixels>::new(1)), "Figure(1)");
}

#[test]
fn cast_unit_test() {
    let pixels = Figure::<u32, Pixels>::new(1);
    let points: Figure<_, Points> = pixels.cast_unit();
    assert_eq!(pixels.get(), points.get());
}

#[test]
fn partial_ord_tests() {
    let one = Figure::<u32, Pixels>::one();
    let zero = Figure::<u32, Pixels>::zero();
    assert!(zero.is_zero());

    assert_eq!(one.min(zero), zero);
    assert_eq!(zero.min(one), zero);
    assert_eq!(one.max(zero), one);
    assert_eq!(zero.max(one), one);
}

#[test]
fn to_primitive_tests() {
    let zero = Figure::<u32, Pixels>::default();

    assert_eq!(zero.to_i8(), Some(0));
    assert_eq!(zero.to_i16(), Some(0));
    assert_eq!(zero.to_i32(), Some(0));
    assert_eq!(zero.to_i64(), Some(0));
    assert_eq!(zero.to_i128(), Some(0));
    assert_eq!(zero.to_isize(), Some(0));
    assert_eq!(zero.to_u8(), Some(0));
    assert_eq!(zero.to_u16(), Some(0));
    assert_eq!(zero.to_u32(), Some(0));
    assert_eq!(zero.to_u64(), Some(0));
    assert_eq!(zero.to_u128(), Some(0));
    assert_eq!(zero.to_usize(), Some(0));
    approx::assert_abs_diff_eq!(zero.to_f32().unwrap(), 0.);
    approx::assert_abs_diff_eq!(zero.to_f64().unwrap(), 0.);
}

#[test]
fn numcast_test() {
    let zero = <Figure<u32, Pixels> as NumCast>::from(0.0_f64);

    assert_eq!(zero.unwrap().get(), 0_u32);
}

#[test]
fn ords_test() {
    let zero = Figure::<u32, Pixels>::zero();
    let one = Figure::<u32, Pixels>::one();

    assert_eq!(zero.cmp(&one), std::cmp::Ordering::Less);
    assert_eq!(zero.partial_cmp(&one), Some(std::cmp::Ordering::Less));
}

#[test]
fn maths_test() {
    let one = Figure::<u32, Pixels>::one();
    assert_eq!(one.get(), 1);
    let two = one + one;
    assert_eq!(two.get(), 2);
    let four = two * two;
    assert_eq!(four.get(), 4);
    let three = four - one;
    assert_eq!(three.get(), 3);
    let one_rem = four % three;
    assert_eq!(one_rem.get(), 1);
    let two_div = four / two;
    assert_eq!(two_div.get(), 2);

    let mut value = one;
    value += one;
    assert_eq!(value.get(), 2);
    value -= one;
    assert_eq!(value.get(), 1);
    value *= four;
    assert_eq!(value.get(), 4);
    value /= two;
    assert_eq!(value.get(), 2);
    value %= two;
    assert_eq!(value.get(), 0);
}

#[test]
fn scalar_maths_test() {
    let one = Figure::<i32, Pixels>::one();
    assert_eq!(one.get(), 1);
    let two = one + 1;
    assert_eq!(two.get(), 2);
    let four = two * 2;
    assert_eq!(four.get(), 4);
    let three = four - 1;
    assert_eq!(three.get(), 3);
    let one_rem = four % 3;
    assert_eq!(one_rem.get(), 1);
    let two_div = four / 2;
    assert_eq!(two_div.get(), 2);
    let neg_two = -two_div;
    assert_eq!(neg_two.get(), -2);

    let mut value = one;
    value += 1;
    assert_eq!(value.get(), 2);
    value -= 1;
    assert_eq!(value.get(), 1);
    value *= 4;
    assert_eq!(value.get(), 4);
    value /= 2;
    assert_eq!(value.get(), 2);
    value %= 2;
    assert_eq!(value.get(), 0);
}

#[test]
fn display_scale_math() {
    let scale = DisplayScale::<u32>::new(Scale::new(2), Scale::new(3));
    let one_scaled = Figure::<u32, Scaled>::one();
    assert_eq!(one_scaled.to_scaled(&scale), one_scaled);
    let two_points = one_scaled.to_points(&scale);
    assert_eq!(two_points.get(), 3);
    assert_eq!(two_points.to_points(&scale), two_points);
    let six_pixels = one_scaled.to_pixels(&scale);
    assert_eq!(six_pixels.get(), 6);
    assert_eq!(six_pixels.to_pixels(&scale), six_pixels);

    assert_eq!(six_pixels.to_points(&scale), two_points);
    assert_eq!(six_pixels.to_scaled(&scale), one_scaled);

    assert_eq!(two_points.to_pixels(&scale), six_pixels);
    assert_eq!(two_points.to_scaled(&scale), one_scaled);
}

#[test]
fn float_ops_test() {
    let one = Figure::<f32, Pixels>::new(1.);
    assert!(Figure::<f32, Pixels>::new(0.5).round().approx_eq(&one));
    assert!(Figure::<f32, Pixels>::new(0.1).ceil().approx_eq(&one));
    assert!(Figure::<f32, Pixels>::new(1.9).floor().approx_eq(&one));
}
