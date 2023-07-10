//! Unsafe [`bytemuck::Pod`] implementations.
//!
//! # Safety
//!
//! Bytemuck prevents deriving `Pod` on any type that contains generics, because
//! it can't ensure that the generic types are tagged `repr(c)`. These
//! implementations are all safe because the types being wrapped all are
//! `repr(c)` and only contain u32/f32/i32.
#![allow(unsafe_code)]

use crate::units::{Dips, Px};
use crate::{Point, Size};

unsafe impl bytemuck::Pod for Point<Px> {}
unsafe impl bytemuck::Zeroable for Point<Px> {}
unsafe impl bytemuck::Pod for Point<Dips> {}
unsafe impl bytemuck::Zeroable for Point<Dips> {}
unsafe impl bytemuck::Pod for Point<i32> {}
unsafe impl bytemuck::Zeroable for Point<i32> {}
unsafe impl bytemuck::Pod for Point<u32> {}
unsafe impl bytemuck::Zeroable for Point<u32> {}
unsafe impl bytemuck::Pod for Point<f32> {}
unsafe impl bytemuck::Zeroable for Point<f32> {}

unsafe impl bytemuck::Pod for Size<Px> {}
unsafe impl bytemuck::Zeroable for Size<Px> {}
unsafe impl bytemuck::Pod for Size<Dips> {}
unsafe impl bytemuck::Zeroable for Size<Dips> {}
unsafe impl bytemuck::Pod for Size<i32> {}
unsafe impl bytemuck::Zeroable for Size<i32> {}
unsafe impl bytemuck::Pod for Size<u32> {}
unsafe impl bytemuck::Zeroable for Size<u32> {}
unsafe impl bytemuck::Pod for Size<f32> {}
unsafe impl bytemuck::Zeroable for Size<f32> {}
