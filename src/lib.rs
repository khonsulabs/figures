#![doc = include_str!("../README.md")]
// This crate uses unsafe, but attempts to minimize its usage. All functions
// that utilize unsafe must explicitly enable it.
#![deny(unsafe_code)]
#![warn(
    // missing_docs,
    clippy::pedantic
)]

#[cfg(feature = "bytemuck")]
mod pod;
mod point;
mod primes;
mod ratio;
mod rect;
mod size;
pub mod traits;
pub mod units;
pub mod utils;

#[cfg(test)]
mod tests;

pub use point::Point;
pub use ratio::Ratio;
pub use rect::Rect;
pub use size::Size;
