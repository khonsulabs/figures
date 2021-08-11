use std::ops::{Mul, Sub};

use crate::{Figure, Point, Size, Vectorlike};

/// A 2d rectangle. This type may internally be represented with a [`SizedRect`]
/// or an [`ExtentsRect`]. All rect types implement [`Rectlike`].
#[derive(Debug)]
pub enum Rect<T, Unit> {
    /// A [`SizedRect`].
    Sized(SizedRect<T, Unit>),
    /// An [`ExtentsRect`].
    Extents(ExtentsRect<T, Unit>),
}

impl<T, Unit> Rect<T, Unit> {
    /// Returns a new rect using `origin` and `size`.
    pub fn sized(origin: Point<T, Unit>, size: Size<T, Unit>) -> Self {
        Self::Sized(SizedRect::new(origin, size))
    }

    /// Returns a new rect using points `origin` and `extent`.
    pub fn extents(origin: Point<T, Unit>, extent: Point<T, Unit>) -> Self {
        Self::Extents(ExtentsRect::new(origin, extent))
    }
}

impl<T, Unit> Copy for Rect<T, Unit> where T: Copy {}

impl<T, Unit> Clone for Rect<T, Unit>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.clone()),
            Self::Extents(extents) => Self::Extents(extents.clone()),
        }
    }
}

/// A rectangle that uses a [`Point`] and a [`Size`] for representation.
#[derive(Debug)]
pub struct SizedRect<T, Unit> {
    /// The origin of the rectangle.
    pub origin: Point<T, Unit>,
    /// The size of the rectangle.
    pub size: Size<T, Unit>,
}

impl<T, Unit> SizedRect<T, Unit> {
    /// Returns a new rectangle using `origin` and `size`.
    pub fn new(origin: Point<T, Unit>, size: Size<T, Unit>) -> Self {
        Self { origin, size }
    }
}

impl<T, Unit> Copy for SizedRect<T, Unit> where T: Copy {}

impl<T, Unit> Clone for SizedRect<T, Unit>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            size: self.size.clone(),
        }
    }
}

/// A rectangle that uses two [`Point`]s for representation.
#[derive(Debug)]
pub struct ExtentsRect<T, Unit> {
    /// The origin of the rectangle.
    pub origin: Point<T, Unit>,
    /// The non-origin point of the rectangle.
    pub extent: Point<T, Unit>,
}

impl<T, Unit> Copy for ExtentsRect<T, Unit> where T: Copy {}

impl<T, Unit> Clone for ExtentsRect<T, Unit>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            extent: self.extent.clone(),
        }
    }
}

impl<T, Unit> ExtentsRect<T, Unit> {
    /// Returns a new rectangle using `origin` and `extent`.
    pub fn new(origin: Point<T, Unit>, extent: Point<T, Unit>) -> Self {
        Self { origin, extent }
    }
}

/// Functionalitiy that all rectangle types implement
pub trait Rectlike<T, Unit>
where
    T: Mul<T, Output = T>,
{
    /// Returns the area contained by this rectangle.
    fn area(&self) -> Figure<T, Unit> {
        self.width() * self.height()
    }

    /// Returns the width of the rectangle.
    fn width(&self) -> Figure<T, Unit>;
    /// Returns the height of the rectangle.
    fn height(&self) -> Figure<T, Unit>;

    /// Returns this rectangle converted to an [`ExtentsRect`].
    fn as_extents(&self) -> ExtentsRect<T, Unit>;
    /// Returns this rectangle converted to a [`SizedRect`].
    fn as_sized(&self) -> SizedRect<T, Unit>;
}

impl<T, Unit> Rectlike<T, Unit> for Rect<T, Unit>
where
    T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    fn width(&self) -> Figure<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.width(),
            Rect::Extents(extents) => extents.width(),
        }
    }

    fn height(&self) -> Figure<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.height(),
            Rect::Extents(extents) => extents.height(),
        }
    }

    fn as_extents(&self) -> ExtentsRect<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.as_extents(),
            Rect::Extents(extents) => extents.as_extents(),
        }
    }

    fn as_sized(&self) -> SizedRect<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.as_sized(),
            Rect::Extents(extents) => extents.as_sized(),
        }
    }
}

impl<T, Unit> Rectlike<T, Unit> for ExtentsRect<T, Unit>
where
    T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    fn width(&self) -> Figure<T, Unit> {
        self.extent.x() - self.origin.x()
    }

    fn height(&self) -> Figure<T, Unit> {
        self.extent.y() - self.origin.y()
    }

    fn as_extents(&self) -> Self {
        *self
    }

    fn as_sized(&self) -> SizedRect<T, Unit> {
        SizedRect::new(
            self.origin,
            (self.extent.to_vector() - self.origin.to_vector()).to_size(),
        )
    }
}

impl<T, Unit> Rectlike<T, Unit> for SizedRect<T, Unit>
where
    T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    fn width(&self) -> Figure<T, Unit> {
        self.size.width()
    }

    fn height(&self) -> Figure<T, Unit> {
        self.size.height()
    }

    fn as_extents(&self) -> ExtentsRect<T, Unit> {
        todo!()
    }

    fn as_sized(&self) -> Self {
        *self
    }
}
