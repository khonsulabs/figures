use std::cmp::Ordering;
use std::fmt;
use std::iter::Peekable;
use std::ops::Mul;

use crate::primes::{FactorsOf, PRIMES};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Ratio {
    numerator: i16,
    denominator: u16,
}

impl Ratio {
    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    #[must_use]
    pub fn new(numerator: i16, denominator: u16) -> Self {
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }

    #[must_use]
    pub const fn numerator(&self) -> i16 {
        self.numerator
    }

    #[must_use]
    pub const fn denominator(&self) -> u16 {
        self.denominator
    }

    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }

    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    #[allow(clippy::cast_possible_truncation)] // truncation desired
    #[must_use]
    pub fn from_f32(scale: f32) -> Self {
        let mut best = Ratio {
            numerator: 0,
            denominator: 0,
        };
        let mut best_diff = f32::MAX;
        for denominator in 1..=u16::MAX {
            let numerator = (f32::from(denominator) * scale) as i16;
            let ratio = Ratio {
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

    #[must_use]
    pub fn into_f32(self) -> f32 {
        f32::from(self.numerator) / f32::from(self.denominator)
    }

    #[must_use]
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub const fn inverse(self) -> Option<Self> {
        if self.denominator < 0x8000 {
            Some(if self.numerator.is_positive() {
                Self {
                    numerator: self.denominator as i16,
                    denominator: self.numerator as u16,
                }
            } else {
                Self {
                    numerator: -(self.denominator as i16),
                    denominator: self.numerator.wrapping_neg() as u16,
                }
            })
        } else {
            None
        }
    }

    fn simplify(mut self) -> Self {
        for prime in PRIMES {
            if let Ok(signed_prime) = i16::try_from(prime) {
                if signed_prime > self.numerator || prime > self.denominator {
                    break;
                }
                if self.numerator % signed_prime == 0 && self.denominator % prime == 0 {
                    self.numerator /= signed_prime;
                    self.denominator /= prime;
                }
            } else {
                break;
            }
        }
        self
    }
}

impl fmt::Debug for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ratio({self})")
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[test]
fn ratio_debug() {
    assert_eq!(format!("{:?}", Ratio::from_f32(1. / 3.)), "Ratio(1/3)");
}

impl Ord for Ratio {
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

#[test]
fn ratio_ord() {
    // Test denominators matching.
    assert!(Ratio::new(1, 3) < Ratio::new(2, 3));
    // Test mismatch in denominators but matching numerator.
    assert!(Ratio::new(1, 3) < Ratio::new(1, 2));

    // Test incompatible fractions
    assert!(Ratio::new(2, 3) > Ratio::new(1, 2));
}

struct LowestCommonDenominator {
    a: FatRatio,
    b: FatRatio,
    a_factors: Peekable<FactorsOf>,
    b_factors: Peekable<FactorsOf>,
}

struct FatRatio {
    numerator: i32,
    denominator: u32,
}

impl From<Ratio> for FatRatio {
    fn from(value: Ratio) -> Self {
        Self {
            numerator: i32::from(value.numerator),
            denominator: u32::from(value.denominator),
        }
    }
}

impl LowestCommonDenominator {
    pub fn find(a: Ratio, b: Ratio) -> (FatRatio, FatRatio) {
        Self {
            a_factors: FactorsOf::new(a.denominator).peekable(),
            b_factors: FactorsOf::new(b.denominator).peekable(),
            a: a.into(),
            b: b.into(),
        }
        .compute()
    }

    fn compute(mut self) -> (FatRatio, FatRatio) {
        loop {
            match (self.a_factors.peek(), self.b_factors.peek()) {
                (Some(a_factor), Some(b_factor)) => {
                    match a_factor.cmp(b_factor) {
                        Ordering::Less => self.apply_a_factor(),
                        Ordering::Equal => {
                            // Factor is already in both
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
        self.b.denominator *= u32::from(a_factor);
        self.b.numerator *= i32::from(a_factor);
    }

    fn apply_b_factor(&mut self) {
        let b_factor = self.b_factors.next().expect("just peeked");
        self.a.denominator *= u32::from(b_factor);
        self.a.numerator *= i32::from(b_factor);
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul<Ratio> for i32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // I guess it is suspicious, lol.
    fn mul(self, rhs: Ratio) -> Self::Output {
        self.saturating_mul(Self::from(rhs.denominator)) / Self::from(rhs.numerator)
    }
}

impl Mul<Ratio> for u32 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // I guess it is suspicious, lol.
    fn mul(self, rhs: Ratio) -> Self::Output {
        if let Ok(numerator) = u32::try_from(rhs.numerator) {
            self.saturating_mul(numerator) / Self::from(rhs.denominator)
        } else {
            0
        }
    }
}

impl Mul<f32> for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs = Self::from_f32(rhs);
        self * rhs
    }
}

impl Mul<Ratio> for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Self {
            numerator: self.numerator.saturating_mul(rhs.numerator),
            denominator: self.denominator.saturating_mul(rhs.denominator),
        }
        .simplify()
    }
}
