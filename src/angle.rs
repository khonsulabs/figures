use std::cmp::Ordering;
use std::fmt::{Debug, Display, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::time::Duration;

use crate::tables::{approximate_via_lookup_table, COSINE_TABLE, SINE_TABLE, TANGENT_TABLE};
use crate::{Fraction, Ranged, Zero};

/// An measurement of distance between two rays sharing a common endpoint, in
/// degrees.
///
/// `Angle::degrees(1)` is equivalent to `1/360th` of a full rotation. This type
/// is commonly used to represent the amount of rotation to perform.
///
/// Internally, this type ensures that the angle represented using a
/// [`Fraction`] is always within the range of `0..=360°` (degrees). This ensures
/// that comparing two angles is efficient and deterministic.
///
/// ```rust
/// use figures::Angle;
///
/// assert_eq!(Angle::degrees(-90).into_degrees::<f32>(), 270.);
/// assert_eq!(Angle::degrees(-90), Angle::degrees(270));
/// ```
///
/// Because this type uses a [`Fraction`] internally, it will perform integer
/// math on integer types. The order of operations may impact the amount of
/// precision the final result contains.
///
/// # Working with Radians
///
/// Because π is an irrational number, this type internally uses degrees for
/// representation. Angles represented in radians can be converted using
/// [`Angle::radians`]/[`Angle::radians_f`].
#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Angle(Fraction);

impl Angle {
    /// Returns an angle for `degrees`, where 360 degrees is equal to one full
    /// rotation.
    ///
    /// The value will be normalized to the range of `0..360`.
    #[must_use]
    pub const fn degrees(mut degrees: i16) -> Self {
        // This implementation of clamp is const, because we can reason about
        // whole numbers better than fractions.
        if degrees < 0 {
            while degrees < 0 {
                degrees += 360;
            }
        } else {
            while degrees > 360 {
                degrees -= 360;
            }
        }
        Self(Fraction::new_whole(degrees))
    }

    /// Returns an angle for `radians`, where `2π` is equal to one full
    /// rotation.
    ///
    /// The value will be normalized to the range of `0..2π`.
    #[must_use]
    pub fn degrees_fraction(degrees: Fraction) -> Self {
        Self(degrees).clamped_to_360()
    }

    /// Returns an angle for `degrees`, where 360 degrees is equal to one full
    /// rotation.
    ///
    /// The value will be normalized to the range of `0..360`.
    #[must_use]
    pub fn degrees_f(degrees: f32) -> Self {
        Self(Fraction::from(degrees)).clamped_to_360()
    }

    /// Returns an angle for `radians`, where `2π` is equal to one full
    /// rotation.
    ///
    /// The value will be normalized to the range of `0..2π`.
    #[must_use]
    pub fn radians(radians: Fraction) -> Self {
        Self(radians / Fraction::PI * 180).clamped_to_360()
    }

    /// Returns an angle for `radians`, where `2π` is equal to one full
    /// rotation.
    ///
    /// The value will be normalized to the range of `0..2π`.
    #[must_use]
    pub fn radians_f(radians: f32) -> Self {
        Self::degrees_f(radians * 180. / std::f32::consts::PI)
    }

    /// Returns this angle as represented in radians, where `2π` is equal to one
    /// full rotation.
    #[must_use]
    pub fn into_raidans<Representation>(self) -> Representation
    where
        Representation: From<Fraction>,
    {
        Representation::from(self.0 / 180 * Fraction::PI)
    }

    /// Returns this angle as represented in radians, where `2π` is equal to one
    /// full rotation.
    #[must_use]
    pub fn into_raidans_f(self) -> f32 {
        f32::from(self.0) / 180. * std::f32::consts::PI
    }

    /// Returns this angle as represented in degrees, where 360 degrees is equal
    /// to one full rotation.
    #[must_use]
    pub fn into_degrees<Representation>(self) -> Representation
    where
        Representation: From<Fraction>,
    {
        Representation::from(self.0)
    }

    fn clamped_to_360(mut self) -> Self {
        self.clamp_to_360();
        self
    }

    fn clamp_to_360(&mut self) {
        const THREESIXTY: Fraction = Fraction::new_whole(360);
        // To check if a ratio is greater than an integer, we might end up doing
        // multiplication and division. Thus, it's better to just do a single
        // division here, and check whether the ratios are still equal.
        match self.0.cmp(&Fraction::ZERO) {
            Ordering::Greater => {
                while self.0 > THREESIXTY {
                    self.0 -= THREESIXTY;
                }
            }
            Ordering::Equal => {}
            Ordering::Less => loop {
                self.0 += THREESIXTY;

                if self.0 > Fraction::ZERO {
                    break;
                }
            },
        }
    }

    /// Calculates the sine of this angle.
    #[must_use]
    pub fn sin(&self) -> Fraction {
        approximate_via_lookup_table(self.0, &SINE_TABLE)
    }

    /// Calculates the cosine of this angle.
    #[must_use]
    pub fn cos(&self) -> Fraction {
        approximate_via_lookup_table(self.0, &COSINE_TABLE)
    }

    /// Calculates the tangent of this angle.
    #[must_use]
    pub fn tan(&self) -> Fraction {
        approximate_via_lookup_table(self.0, &TANGENT_TABLE)
    }
}

impl Ranged for Angle {
    const MAX: Self = Self(Fraction::new_whole(360));
    const MIN: Self = Self::ZERO;
}

impl Zero for Angle {
    const ZERO: Self = Self(Fraction::ZERO);

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Self::radians_f(value)
    }
}

impl From<Fraction> for Angle {
    fn from(value: Fraction) -> Self {
        Self::radians(value)
    }
}

impl From<i16> for Angle {
    fn from(value: i16) -> Self {
        Self::degrees(value)
    }
}

impl Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0.3}\u{B0}", self.0.into_f32())
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (whole, mut fraction) = self.0.into_compound();
        let is_non_negative = !whole.is_negative();
        let whole = whole.to_string();
        f.pad_integral(is_non_negative, "", &whole)?;
        if !fraction.is_zero() {
            if let Some(precision) = f.precision() {
                f.write_char('.')?;
                for _ in 0..precision {
                    let (digit, remainder) = (fraction * Fraction::new_whole(10)).into_compound();
                    f.write_char(char::from(
                        b'0' + u8::try_from(digit).expect("fractional value"),
                    ))?;
                    fraction = remainder;
                }
            } else if fraction > Fraction::new(1, 1000) {
                f.write_char('.')?;
                loop {
                    let (digit, remainder) = (fraction * Fraction::new_whole(10)).into_compound();
                    f.write_char(char::from(
                        b'0' + u8::try_from(digit).expect("fractional value"),
                    ))?;
                    fraction = remainder;
                    if fraction < Fraction::new(1, 1000) {
                        break;
                    }
                }
            }
        }

        f.write_str("°")
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0).clamped_to_360()
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.clamp_to_360();
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0).clamped_to_360()
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.clamp_to_360();
    }
}

impl Div for Angle {
    type Output = Angle;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0).clamped_to_360()
    }
}

impl DivAssign for Angle {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.clamp_to_360();
    }
}

impl Mul for Angle {
    type Output = Angle;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0).clamped_to_360()
    }
}

impl MulAssign for Angle {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.clamp_to_360();
    }
}

impl Mul<Duration> for Angle {
    type Output = Angle;

    fn mul(self, rhs: Duration) -> Self::Output {
        Self(self.0 * rhs.as_secs_f32()).clamped_to_360()
    }
}

macro_rules! impl_math_ops_for_std_type {
    ($type:ident) => {
        impl Add<$type> for Angle {
            type Output = Angle;

            fn add(self, rhs: $type) -> Self::Output {
                Self(self.0 + rhs).clamped_to_360()
            }
        }

        impl AddAssign<$type> for Angle {
            fn add_assign(&mut self, rhs: $type) {
                self.0 += rhs;
                self.clamp_to_360();
            }
        }

        impl Sub<$type> for Angle {
            type Output = Angle;

            fn sub(self, rhs: $type) -> Self::Output {
                Self(self.0 - rhs).clamped_to_360()
            }
        }

        impl SubAssign<$type> for Angle {
            fn sub_assign(&mut self, rhs: $type) {
                self.0 -= rhs;
                self.clamp_to_360();
            }
        }

        impl Div<$type> for Angle {
            type Output = Angle;

            fn div(self, rhs: $type) -> Self::Output {
                Self(self.0 / rhs).clamped_to_360()
            }
        }

        impl DivAssign<$type> for Angle {
            fn div_assign(&mut self, rhs: $type) {
                self.0 /= rhs;
                self.clamp_to_360();
            }
        }

        impl Mul<$type> for Angle {
            type Output = Angle;

            fn mul(self, rhs: $type) -> Self::Output {
                Self(self.0 * rhs).clamped_to_360()
            }
        }

        impl MulAssign<$type> for Angle {
            fn mul_assign(&mut self, rhs: $type) {
                self.0 *= rhs;
                self.clamp_to_360();
            }
        }
    };
}

impl_math_ops_for_std_type!(f32);
impl_math_ops_for_std_type!(i16);

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[test]
fn angle_clamp() {
    assert_eq!(Angle::degrees(361), Angle::degrees(1));
    assert_eq!(Angle::degrees(-1), Angle::degrees(359));
    assert_eq!(Angle::degrees_f(361.), Angle::degrees_f(1.));
    assert_eq!(Angle::degrees_f(-1.), Angle::degrees_f(359.));
}

#[test]
fn angle_display() {
    assert_eq!(format!("{}", Angle::degrees(10)), "10°");
    assert_eq!(format!("{}", Angle::degrees_f(10.1001)), "10.1°");
    assert_eq!(format!("{}", Angle::degrees_f(10.101)), "10.101°");
    assert_eq!(format!("{}", Angle::degrees_f(0.125)), "0.125°");
    assert_eq!(format!("{:.3}", Angle::degrees(1)), "1°");
    assert_eq!(format!("{:.3}", Angle::degrees_f(1.1)), "1.100°");
    assert_eq!(format!("{:.3}", Angle::degrees_f(0.125)), "0.125°");
}

#[test]
fn radians_to_deg() {
    assert_eq!(Angle::radians(Fraction::PI), Angle::degrees(180));
    assert_eq!(Angle::degrees(180).into_raidans::<Fraction>(), Fraction::PI);
    assert_eq!(Angle::radians_f(std::f32::consts::PI), Angle::degrees(180));
}

#[test]
fn trig_approximation() {
    use std::f32::consts::PI;

    #[track_caller]
    fn assert_close_enough(f1: Fraction, f2: f32) {
        println!("Comparing {f1} against {f2}");
        assert!(
            (f1.into_f32() - f2).abs() < 0.000_001,
            "{f1} ({}) is not close enough to {f2}",
            f1.into_f32()
        );
    }
    // We use a lookup table for the whole portion of the degrees, and then
    // approximate the remainder by using the delta between the current degree
    // and the "next".
    assert_close_enough(Angle::degrees(0).sin(), (0. / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(0.25).sin(), (0.25 / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(0.5).sin(), (0.5 / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(0.75).sin(), (0.75 / 180. * PI).sin());
    assert_close_enough(Angle::degrees(359).sin(), (359. / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(359.25).sin(), (359.25 / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(359.5).sin(), (359.5 / 180. * PI).sin());
    assert_close_enough(Angle::degrees_f(359.75).sin(), (359.75 / 180. * PI).sin());
}
