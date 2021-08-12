//! A small math library specialized for 2d screen graphics.
//!
//! ## Feature Flags
//!
//! To enable serialization of most types, enable the `serde` feature flag.

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![allow(clippy::if_not_else, clippy::module_name_repetitions)]
#![cfg_attr(doc, warn(rustdoc::all))]

mod angle;
mod approxeq;
mod displayable;
mod figure;
mod num;
mod point;
mod rect;
mod scale;
mod size;
mod vector;
mod vectorlike;

pub use approx;
pub use num_traits;

pub use self::{
    angle::Angle,
    approxeq::Approx,
    displayable::{DisplayScale, Displayable, Pixels, Points, Scaled},
    figure::Figure,
    num::{Ceil, Floor, One, Round, Zero},
    rect::{ExtentsRect, Rect, Rectlike, SizedRect},
    scale::Scale,
    vectorlike::{Point, Size, Vector, Vectorlike},
};
