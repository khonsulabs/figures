#![doc = include_str!("../README.md")]
// This crate uses unsafe, but attempts to minimize its usage. All functions
// that utilize unsafe must explicitly enable it.
#![deny(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
mod fraction;
#[macro_use]
mod twod;
#[cfg(feature = "bytemuck")]
mod pod;
mod point;
mod primes;
mod rect;
mod size;
mod tables;
mod traits;
pub use traits::{
    FloatConversion, FloatOrInt, FromComponents, IntoComponents, IntoSigned, IntoUnsigned, Lp2D,
    PixelScaling, Px2D, Ranged, Roots, Round, ScreenScale, ScreenUnit, UPx2D, Unit, UnscaledUnit,
    Zero,
};
/// The measurement units supported by figures.
pub mod units;
/// Utility functions to aide in warning-free development for users of
/// `clippy::pedantic`.
mod utils;

mod angle;
#[cfg(test)]
mod tests;

pub use angle::Angle;
pub use fraction::Fraction;
pub use point::Point;
pub use rect::Rect;
pub use size::Size;
