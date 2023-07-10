use std::cmp::Ordering;
use std::fmt;
use std::iter::Peekable;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use crate::primes::{FactorsOf, PRIMES};
use crate::tables::{approximate_via_lookup_table, ARCTAN_SUBDIVISIONS, ARCTAN_TABLE};
use crate::traits::IsZero;
use crate::Angle;

/// Returns a new fraction.
///
/// This macro has two forms:
///
/// - Whole numbers:
///
///   ```rust
///   use figures::{fraction, Fraction};
///   assert_eq!(fraction!(42), Fraction::new_whole(42));
///   ```
/// - Fractions:
///
///   ```rust
///   use figures::{fraction, Fraction};
///   assert_eq!(fraction!(42/7), Fraction::new(42, 7));
///   ```
#[macro_export]
macro_rules! fraction {
    ($numerator:literal) => {
        $crate::Fraction::new_whole($numerator)
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::Fraction::new($numerator, $denominator)
    };
}

/// A fraction type for predictable integer-based math.
///
/// Internally this type uses 32 bits of data to represent a fraction:
///
/// - 1 bit of data for the positive/negative sign.
/// - 15 bits of data for the numerator
/// - 16 bits of data for the denominator
///
/// Many math operations are performed using temporary 32-bit values for the
/// fraction, simplifing at the end of the operation. This prevents overflows,
/// but does not prevent precision loss. We can see this by purposely buliding
/// fractions that are hard to represent:
///
/// ```rust
/// use figures::fraction;
///
/// assert_eq!(
///     fraction!(1 / 32719) + fraction!(1 / 32749),
///     fraction!(2 / 32749)
/// );
/// ```
///
/// The above example adds fractions that have denominators using two largest
/// primes that fit in 16 bits. The result should be `65,468/1,071,514,531`, but
/// that denominator is clearly too large. Because the fractions being added had
/// prime numbers for their denominators, there is no way to reduce this
/// fraction without losing information. For this particular example, a
/// precision loss of ~`2.8e-8` occurs.
///
/// However, in 2d graphics programming, it's rare to be working with irrational
/// numbers outside of angles represented in radians.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Fraction {
    numerator: i16,
    denominator: i16,
}

const MIN_VALUE: i16 = -i16::MAX;

impl From<f32> for Fraction {
    #[allow(clippy::cast_possible_truncation)] // truncation desired
    fn from(scale: f32) -> Self {
        if scale < f32::from(MIN_VALUE) {
            Self::MIN
        } else if scale > f32::from(i16::MAX) {
            Self::MAX
        } else {
            let mut best = Fraction {
                numerator: 0,
                denominator: 0,
            };
            let mut best_diff = f32::MAX;
            for denominator in 1..=i16::MAX {
                let numerator = (f32::from(denominator) * scale).round() as i16;
                let ratio = Fraction {
                    numerator,
                    denominator,
                };
                let delta = (ratio.into_f32() - scale).abs();
                if delta < best_diff {
                    best = ratio;
                    best_diff = delta;
                    if delta <= f32::EPSILON {
                        break;
                    }
                }
            }

            best
        }
    }
}

impl From<Fraction32> for Fraction {
    fn from(
        Fraction32 {
            mut numerator,
            mut denominator,
        }: Fraction32,
    ) -> Self {
        reduce(&mut numerator, &mut denominator);
        if let (Ok(numerator), Ok(denominator)) =
            (i16::try_from(numerator), i16::try_from(denominator))
        {
            if numerator >= MIN_VALUE {
                return Self::new_maybe_reduced(numerator, denominator);
            }
        }

        // Reducing didn't yield a fraction that we can represent perfectly.
        // Hunt for the largest prime divisor that yields a usable fraction
        // and the smallest remainder.
        let mut best_numerator = i16::MAX;
        let mut best_denominator = i16::MAX;
        let mut best_remainder = i32::MAX;
        for prime in PRIMES
            .iter()
            .rev()
            .map(|&prime| i32::from(prime))
            .filter(|&prime| numerator >= prime && denominator >= prime)
        {
            let numerator_remainder = numerator % prime;
            let Ok(numerator) = i16::try_from(numerator / prime) else { break };
            if numerator < MIN_VALUE {
                break;
            }
            let denominator_remainder = denominator % prime;
            let Ok(denominator) = i16::try_from(denominator / prime) else { break };
            let remainder = numerator_remainder + denominator_remainder;
            if remainder < best_remainder {
                best_numerator = numerator;
                best_denominator = denominator;
                best_remainder = remainder;
                if remainder <= 5 {
                    break;
                }
            }
        }
        Self {
            numerator: best_numerator,
            denominator: best_denominator,
        }
    }
}

impl From<i16> for Fraction {
    fn from(numerator: i16) -> Self {
        Self {
            numerator,
            denominator: 1,
        }
    }
}

impl From<Fraction> for f32 {
    fn from(value: Fraction) -> Self {
        value.into_f32()
    }
}

macro_rules! try_from_int {
    ($type:ident) => {
        impl TryFrom<$type> for Fraction {
            type Error = TryFromIntError;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                i16::try_from(value).map(Self::from)
            }
        }
    };
}

try_from_int!(i32);
try_from_int!(i64);
try_from_int!(i128);
try_from_int!(isize);
try_from_int!(u16);
try_from_int!(u32);
try_from_int!(u64);
try_from_int!(u128);
try_from_int!(usize);

impl Fraction {
    /// The maximum value representable by this type.
    pub const MAX: Self = Self::new_whole(i16::MAX);
    /// The minimum value representable by this type.
    pub const MIN: Self = Self::new_whole(i16::MIN);
    /// A fraction equivalent to 1.
    pub const ONE: Self = Self::new_whole(1);
    /// A fractional approximation of Pi, accurate to within 2.67e-7.
    pub const PI: Self = Self::new_maybe_reduced(355, 113);
    /// A fraction equivalent to 0.
    pub const ZERO: Self = Self::new_maybe_reduced(0, 1);

    /// Returns a new fraction for a whole number.
    #[must_use]
    pub const fn new_whole(whole_number: i16) -> Self {
        Self {
            numerator: whole_number,
            denominator: 1,
        }
    }

    pub(crate) const fn new_maybe_reduced(mut numerator: i16, mut denominator: i16) -> Self {
        if denominator.is_negative() {
            numerator = numerator.saturating_neg();
            denominator = denominator.saturating_neg();
        }
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns a new fraction using the components provided.
    ///
    /// `denominator` will be limited to the absolute value of `i16::MIN`.
    #[must_use]
    pub fn new(numerator: i16, denominator: i16) -> Self {
        Self::new_maybe_reduced(numerator.max(MIN_VALUE), denominator).reduce()
    }

    /// Returns the numerator of the fraction.
    #[must_use]
    pub const fn numerator(&self) -> i16 {
        self.numerator
    }

    /// Returns the denominator of the fraction.
    #[must_use]
    pub const fn denominator(&self) -> i16 {
        self.denominator
    }

    /// Returns true if the fraction is positive (greater than zero).
    ///
    /// Note: Zero is neither negative nor positive.
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    /// Returns true if the fraction is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    /// Returns true if the fraction is negative (less than zero).
    ///
    /// Note: Zero is neither negative nor positive.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    /// Simplifies the fraction into a compound number.
    ///
    /// ```rust
    /// use figures::Fraction;
    ///
    /// assert_eq!(
    ///     Fraction::new(1, 3).into_compound(),
    ///     (0, Fraction::new(1, 3))
    /// );
    /// assert_eq!(
    ///     Fraction::new(4, 3).into_compound(),
    ///     (1, Fraction::new(1, 3))
    /// );
    /// assert_eq!(
    ///     Fraction::new(-4, 3).into_compound(),
    ///     (-1, Fraction::new(-1, 3))
    /// );
    /// ```
    ///
    /// Adding the number and the fraction back together will result in the
    /// original fraction.
    ///
    /// ```rust
    /// use figures::Fraction;
    ///
    /// let improper = Fraction::new(-4, 3);
    /// let (whole, fraction) = improper.into_compound();
    /// assert_eq!(Fraction::from(whole) + fraction, improper);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn into_compound(self) -> (i16, Fraction) {
        let clamped_denominator = self.denominator;
        let whole = self.numerator / clamped_denominator;
        let numerator = self.numerator % clamped_denominator;
        (
            whole,
            Fraction::new_maybe_reduced(numerator, self.denominator),
        )
    }

    /// Rounds this fraction to the nearest whole number.
    #[must_use]
    pub fn round(self) -> i16 {
        self.round_with_amount().0
    }

    /// Rounds this fraction to the nearest whole number, returning the
    /// fractional component that was rounded away. This fraction can be added
    /// to the whole number to reconstruct the original fraction.
    ///
    /// ```rust
    /// use figures::Fraction;
    ///
    /// let (whole, fraction) = Fraction::new(5, 3).round_with_amount();
    /// assert_eq!(whole, 2);
    /// assert_eq!(fraction, Fraction::new(-1, 3));
    /// assert_eq!(Fraction::new_whole(whole) + fraction, Fraction::new(5, 3));
    ///
    /// let (whole, fraction) = Fraction::new(-5, 3).round_with_amount();
    /// assert_eq!(whole, -2);
    /// assert_eq!(fraction, Fraction::new(1, 3));
    /// assert_eq!(Fraction::new_whole(whole) + fraction, Fraction::new(-5, 3));
    /// ```
    #[must_use]
    pub fn round_with_amount(self) -> (i16, Fraction) {
        let (whole, fraction) = self.into_compound();
        let half_denominator = (fraction.denominator + 1) / 2;
        if fraction.numerator >= half_denominator {
            (whole + 1, -(Fraction::new_whole(1) - fraction))
        } else if fraction.numerator <= -half_denominator {
            (whole - 1, -(Fraction::new_whole(-1) - fraction))
        } else {
            (whole, fraction)
        }
    }

    /// Returns this fraction as a floating point number.
    #[must_use]
    pub fn into_f32(self) -> f32 {
        f32::from(self.numerator) / f32::from(self.denominator)
    }

    /// Returns the inverse of this fraction.
    #[must_use]
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub const fn inverse(self) -> Self {
        if self.numerator >= 0 {
            Self {
                numerator: self.denominator,
                denominator: self.numerator,
            }
        } else {
            Self {
                numerator: -self.denominator,
                denominator: -self.numerator,
            }
        }
    }

    fn reduce(mut self) -> Self {
        reduce(&mut self.numerator, &mut self.denominator);
        self
    }

    /// Returns the absolute value of this fraction.
    #[must_use]
    pub const fn abs(self) -> Self {
        if self.numerator >= 0 {
            self
        } else {
            Self {
                numerator: -self.numerator,
                denominator: self.denominator,
            }
        }
    }

    /// Returns the arctangent of this fraction.
    ///
    /// This function is implemented using a lookup table and is an
    /// approximation.
    #[must_use]
    pub fn atan(self) -> Angle {
        if self >= Fraction::new_whole(-1) && self <= Fraction::new_whole(1) {
            self.fast_atan()
        } else {
            let inverse_atan = self.inverse().fast_atan();
            if self.is_negative() {
                Angle::degrees(-90) - inverse_atan
            } else {
                Angle::degrees(90) - inverse_atan
            }
        }
    }

    /// Returns the result of arctan(self/other) while correctly handling
    /// negative numbers.
    #[must_use]
    pub fn atan2(self, other: Self) -> Angle {
        let x = other;
        let y = self;
        let atan = (y / x).atan();
        if x.is_negative() {
            if y.is_negative() {
                -Angle::degrees(180) + atan // 3rd quadrant
            } else {
                Angle::degrees(180) + atan // 2nd quadrant
            }
        } else {
            atan // 1st/4th quadrant
        }
    }

    fn fast_atan(self) -> Angle {
        let index = self * Self::new_whole(ARCTAN_SUBDIVISIONS);
        let result = if index.is_negative() {
            -approximate_via_lookup_table(-index, &ARCTAN_TABLE)
        } else {
            approximate_via_lookup_table(index, &ARCTAN_TABLE)
        };
        Angle::radians(result)
    }
}

#[test]
fn atan() {
    assert_eq!(Fraction::new_whole(1).atan(), Angle::degrees(45));
    assert_eq!(Fraction::new_whole(0).atan(), Angle::degrees(0));
    assert_eq!(Fraction::new_whole(-1).atan(), Angle::degrees(315));
}

#[test]
fn atan2() {
    assert_eq!(
        Fraction::new_whole(1).atan2(Fraction::new_whole(1)),
        Angle::degrees(45)
    );
    assert_eq!(
        Fraction::new_whole(1).atan2(Fraction::new_whole(-1)),
        Angle::degrees(135)
    );
    assert_eq!(
        Fraction::new_whole(-1).atan2(Fraction::new_whole(-1)),
        Angle::degrees(225)
    );
    assert_eq!(
        Fraction::new_whole(-1).atan2(Fraction::new_whole(1)),
        Angle::degrees(315)
    );
}

pub fn reduce<T>(numerator: &mut T, denominator: &mut T)
where
    T: Abs + IsZero + Copy + From<i16> + Ord + Rem<Output = T> + DivAssign,
{
    let one = T::from(1);
    if numerator.is_zero() {
        *denominator = one;
    } else if *denominator > one {
        for prime in PRIMES {
            let prime = T::from(prime);
            if prime > numerator.abs() || prime > *denominator {
                break;
            }
            while (*numerator % prime).is_zero() && (*denominator % prime).is_zero() {
                *numerator /= prime;
                *denominator /= prime;
                if *denominator == one {
                    break;
                }
            }
        }
    }
}

pub trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for i32 {
    fn abs(&self) -> Self {
        self.wrapping_abs()
    }
}

impl Abs for i16 {
    fn abs(&self) -> Self {
        self.wrapping_abs()
    }
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fraction({self})")
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.denominator == other.denominator {
            // Denominators match
            self.numerator.cmp(&other.numerator)
        } else if self.numerator == other.numerator {
            // Numerators match. The comparison is the inverse of the result of
            // comparing the denominators.
            other.denominator.cmp(&self.denominator)
        } else {
            // To compare these ratios, we must find the lowest common
            // denominator.
            let (this, other) = LowestCommonDenominator::find(*self, *other);
            debug_assert_eq!(this.denominator, other.denominator);
            this.numerator.cmp(&other.numerator)
        }
    }
}

struct LowestCommonDenominator {
    a: Fraction32,
    b: Fraction32,
    a_factors: Peekable<FactorsOf>,
    b_factors: Peekable<FactorsOf>,
}

#[derive(Clone, Copy, Debug)]
pub struct Fraction32 {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction32 {
    pub const fn inverse(self) -> Self {
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
}

impl From<Fraction> for Fraction32 {
    fn from(value: Fraction) -> Self {
        Self {
            numerator: i32::from(value.numerator),
            denominator: i32::from(value.denominator),
        }
    }
}

impl From<i32> for Fraction32 {
    fn from(numerator: i32) -> Self {
        Self {
            numerator,
            denominator: 1,
        }
    }
}

impl Add for Fraction32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut this, rhs) = LowestCommonDenominator::find32(self, rhs);
        let mut numerator = this.numerator + rhs.numerator;
        reduce(&mut numerator, &mut this.denominator);
        Self {
            numerator,
            denominator: this.denominator,
        }
    }
}

impl Sub for Fraction32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (mut this, rhs) = LowestCommonDenominator::find32(self, rhs);
        let mut numerator = this.numerator - rhs.numerator;
        reduce(&mut numerator, &mut this.denominator);
        Self {
            numerator,
            denominator: this.denominator,
        }
    }
}

impl Mul for Fraction32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut numerator = self.numerator * rhs.numerator;
        let mut denominator = self.denominator * rhs.denominator;
        reduce(&mut numerator, &mut denominator);
        Self {
            numerator,
            denominator,
        }
    }
}

impl Div for Fraction32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl LowestCommonDenominator {
    pub fn find(a: Fraction, b: Fraction) -> (Fraction32, Fraction32) {
        Self {
            a_factors: FactorsOf::new(a.denominator).peekable(),
            b_factors: FactorsOf::new(b.denominator).peekable(),
            a: a.into(),
            b: b.into(),
        }
        .compute()
    }

    pub fn find32(a: Fraction32, b: Fraction32) -> (Fraction32, Fraction32) {
        if a.denominator == b.denominator {
            (a, b)
        } else {
            Self {
                a_factors: FactorsOf::new(a.denominator).peekable(),
                b_factors: FactorsOf::new(b.denominator).peekable(),
                a,
                b,
            }
            .compute()
        }
    }

    fn compute(mut self) -> (Fraction32, Fraction32) {
        loop {
            match (self.a_factors.peek(), self.b_factors.peek()) {
                (Some(a_factor), Some(b_factor)) => {
                    match a_factor.cmp(b_factor) {
                        Ordering::Less => self.apply_a_factor(),
                        Ordering::Equal => {
                            // Factor is already in both.
                            self.a_factors.next();
                            self.b_factors.next();
                        }
                        Ordering::Greater => self.apply_b_factor(),
                    }
                }
                (None, Some(_)) => self.apply_b_factor(),
                (Some(_), None) => self.apply_a_factor(),
                (None, None) => break,
            }
        }

        (self.a, self.b)
    }

    fn apply_a_factor(&mut self) {
        let a_factor = self.a_factors.next().expect("just peeked");
        self.b.denominator *= i32::from(a_factor);
        self.b.numerator *= i32::from(a_factor);
    }

    fn apply_b_factor(&mut self) {
        let b_factor = self.b_factors.next().expect("just peeked");
        self.a.denominator *= i32::from(b_factor);
        self.a.numerator *= i32::from(b_factor);
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        let (this, rhs) = LowestCommonDenominator::find(*self, rhs);
        *self = Self::from(this + rhs);
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (this, rhs) = LowestCommonDenominator::find(self, rhs);
        Self::from(this - rhs)
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        let (this, rhs) = LowestCommonDenominator::find(*self, rhs);
        *self = Self::from(this - rhs);
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Expand to 32-bits for the multiplication, then reduce.
        let numerator = i32::from(self.numerator) * i32::from(rhs.numerator);
        let denominator = i32::from(self.denominator) * i32::from(rhs.denominator);
        Self::from(Fraction32 {
            numerator,
            denominator,
        })
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Fraction> for i32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Fraction) -> Self::Output {
        self.saturating_mul(Self::from(rhs.denominator)) / Self::from(rhs.numerator)
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Mul<Fraction> for u32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Fraction) -> Self::Output {
        if let (Ok(numerator), Ok(denominator)) =
            (u32::try_from(rhs.numerator), u32::try_from(rhs.denominator))
        {
            self.saturating_mul(numerator) / denominator
        } else {
            0
        }
    }
}

impl Div for Fraction {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // I guess it is suspicious, lol.
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Div<Fraction> for i32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // I guess it is suspicious, lol.
    fn div(self, rhs: Fraction) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Div<Fraction> for u32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // I guess it is suspicious, lol.
    fn div(self, rhs: Fraction) -> Self::Output {
        self * rhs.inverse()
    }
}

macro_rules! impl_math_ops_for_std_type {
    ($type:ident) => {
        impl Add<$type> for Fraction {
            type Output = Fraction;

            fn add(self, rhs: $type) -> Self::Output {
                self + Self::from(rhs)
            }
        }

        impl AddAssign<$type> for Fraction {
            fn add_assign(&mut self, rhs: $type) {
                *self += Self::from(rhs);
            }
        }

        impl Sub<$type> for Fraction {
            type Output = Fraction;

            fn sub(self, rhs: $type) -> Self::Output {
                self - Self::from(rhs)
            }
        }

        impl SubAssign<$type> for Fraction {
            fn sub_assign(&mut self, rhs: $type) {
                *self -= Self::from(rhs);
            }
        }

        impl Div<$type> for Fraction {
            type Output = Fraction;

            fn div(self, rhs: $type) -> Self::Output {
                self / Self::from(rhs)
            }
        }

        impl DivAssign<$type> for Fraction {
            fn div_assign(&mut self, rhs: $type) {
                *self /= Self::from(rhs);
            }
        }

        impl Mul<$type> for Fraction {
            type Output = Fraction;

            fn mul(self, rhs: $type) -> Self::Output {
                self * Self::from(rhs)
            }
        }

        impl MulAssign<$type> for Fraction {
            fn mul_assign(&mut self, rhs: $type) {
                *self *= Self::from(rhs);
            }
        }
    };
}

impl_math_ops_for_std_type!(f32);
impl_math_ops_for_std_type!(i16);

#[test]
fn ratio_ord() {
    // Test denominators matching.
    assert!(Fraction::new(1, 3) < Fraction::new(2, 3));
    // Test mismatch in denominators but matching numerator.
    assert!(Fraction::new(1, 3) < Fraction::new(1, 2));

    // Test incompatible fractions
    assert!(Fraction::new(2, 3) > Fraction::new(1, 2));
}

#[test]
fn ratio_debug() {
    assert_eq!(format!("{:?}", Fraction::from(1. / 3.)), "Fraction(1/3)");
}

#[test]
fn pi() {
    assert_eq!(Fraction::from(std::f32::consts::PI), Fraction::PI);
}

#[test]
fn math() {
    assert_eq!(
        Fraction::new(2, 3) * Fraction::new(3, 4),
        Fraction::new(1, 2)
    );
    assert_eq!(
        Fraction::new(2, 3) / Fraction::new(2, 1),
        Fraction::new(1, 3)
    );
}
#[test]
fn lossy_simplification() {
    assert_eq!(
        fraction!(1 / 32_767) + fraction!(1 / 32_767),
        fraction!(2 / 32_767)
    );
}

#[test]
fn compound_signs() {
    assert_eq!(fraction!(-1 / 3).into_compound(), (0, Fraction::new(-1, 3)));
    assert_eq!(
        fraction!(-4 / 3).into_compound(),
        (-1, Fraction::new(-1, 3))
    );
    assert_eq!(fraction!(4 / 3).into_compound(), (1, Fraction::new(1, 3)));
}

#[test]
fn negative_denominator() {
    assert_eq!(
        fraction!(1 / -3),
        Fraction {
            numerator: -1,
            denominator: 3
        }
    );
    assert_eq!(
        fraction!(-1 / -3),
        Fraction {
            numerator: 1,
            denominator: 3
        }
    );
}
