//! A math library specialized for 2d screen graphics.

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

mod displayable;
mod figure;
mod point;
mod rect;
mod scale;
mod size;
mod vector;
#[macro_use]
mod vectorlike;
mod angle;
mod num;

pub use num_traits;

pub use self::{
    angle::Angle,
    displayable::{
        DisplayScale, Displayable, Pixels, Points, Scaled, ToPixels, ToPoints, ToScaled,
    },
    figure::Figure,
    num::{Ceil, Floor, One, Round, Zero},
    rect::{ExtentsRect, Rect, Rectlike, SizedRect},
    scale::Scale,
    vectorlike::{Point, Size, Vector, Vectorlike},
};
