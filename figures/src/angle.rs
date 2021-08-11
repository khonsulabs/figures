use std::{
    fmt::{Debug, Write},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num_traits::Float;

use crate::{Point, Vector};

/// An angle of rotation.
#[derive(Clone, Copy)]
pub enum Angle<T> {
    /// An angle expressed in radians.
    Radians(T),
    /// An angle expressed in degrees.
    Degrees(T),
}

impl<T: Default> Default for Angle<T> {
    fn default() -> Self {
        Self::Radians(T::default())
    }
}

impl<T: Float> Angle<T> {
    /// Returns this angle, converting to degrees if necessary.
    pub fn degrees(&self) -> T {
        match self {
            Self::Radians(theta) => theta.to_degrees(),
            Self::Degrees(theta) => *theta,
        }
    }

    /// Returns this angle, converting to radians if necessary.
    pub fn radians(&self) -> T {
        match self {
            Self::Radians(theta) => *theta,
            Self::Degrees(theta) => theta.to_radians(),
        }
    }

    /// Transforms `point` around 0,0 by this angle.
    pub fn transform_point<Unit>(&self, point: Point<T, Unit>) -> Point<T, Unit> {
        let (x, y) = rotate(self.radians(), point.x, point.y);
        Point::new(x, y)
    }

    /// Transforms `vector` around 0,0 by this angle.
    pub fn transform_vector<Unit>(&self, vector: Vector<T, Unit>) -> Vector<T, Unit> {
        let (x, y) = rotate(self.radians(), vector.x, vector.y);
        Vector::new(x, y)
    }
}

fn rotate<T: Float>(radians: T, x: T, y: T) -> (T, T) {
    let cos = radians.cos();
    let sin = radians.sin();
    (x * cos - y * sin, y * cos + x * sin)
}

impl<T: Debug> Debug for Angle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Angle(")?;

        match self {
            Self::Radians(theta) => write!(f, "{:?} rad", theta)?,
            Self::Degrees(theta) => write!(f, "{:?} deg", theta)?,
        }

        f.write_char(')')
    }
}

impl<T> Add for Angle<T>
where
    T: Float,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Radians(radians) => Angle::Radians(radians + rhs.radians()),
            Self::Degrees(degrees) => Self::Degrees(degrees + rhs.degrees()),
        }
    }
}

impl<T> AddAssign for Angle<T>
where
    T: Float + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Self::Radians(radians) => *radians += rhs.radians(),
            Self::Degrees(degrees) => *degrees += rhs.degrees(),
        }
    }
}

impl<T> Sub for Angle<T>
where
    T: Float,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Radians(radians) => Angle::Radians(radians - rhs.radians()),
            Self::Degrees(degrees) => Self::Degrees(degrees - rhs.degrees()),
        }
    }
}

impl<T> SubAssign for Angle<T>
where
    T: Float + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Self::Radians(radians) => *radians -= rhs.radians(),
            Self::Degrees(degrees) => *degrees -= rhs.degrees(),
        }
    }
}
