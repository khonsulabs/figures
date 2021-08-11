use std::ops::{Add, Div, Mul, Sub};

use num_traits::{NumCast, One, Zero};

use crate::{Figure, Point, Round, Scale, Size, Vectorlike};

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

impl<T, Unit> Rect<T, Unit>
where
    T: NumCast + Copy,
{
    /// Attempts to cast `T` to `NewT`. If unsuccessful, None is returned.
    pub fn try_cast<NewT: NumCast + Copy>(&self) -> Option<Rect<NewT, Unit>> {
        Some(match self {
            Self::Sized(sized) => Rect::Sized(sized.try_cast()?),
            Self::Extents(extents) => Rect::Extents(extents.try_cast()?),
        })
    }

    /// Casts `T` to `NewT`.
    ///
    /// # Panics
    ///
    /// Panics if casting fails.
    pub fn cast<NewT: NumCast + Copy>(&self) -> Rect<NewT, Unit> {
        self.try_cast().expect("unable to cast")
    }
}

impl<T, Unit> Rect<T, Unit>
where
    T: Copy,
{
    /// Returns this value with the new unit. Does not affect the underlying
    /// value.
    pub fn cast_unit<NewUnit>(&self) -> Rect<T, NewUnit> {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.cast_unit()),
            Self::Extents(extents) => Rect::Extents(extents.cast_unit()),
        }
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

impl<T, Unit> Eq for Rect<T, Unit> where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy
{
}

impl<T, Unit> PartialEq for Rect<T, Unit>
where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Sized(sized) => sized.eq(&other.as_sized()),
            Self::Extents(extents) => extents.eq(&other.as_extents()),
        }
    }
}

impl<T, Unit> Default for Rect<T, Unit>
where
    T: Default,
{
    fn default() -> Self {
        Self::Sized(SizedRect::default())
    }
}

impl<T, Unit> Round for Rect<T, Unit>
where
    T: Round,
{
    fn round(self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.round()),
            Self::Extents(extents) => Self::Extents(extents.round()),
        }
    }
}

impl<T, UnitA, UnitB> Mul<Scale<T, UnitA, UnitB>> for Rect<T, UnitA>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Rect<T, UnitB>;

    fn mul(self, rhs: Scale<T, UnitA, UnitB>) -> Self::Output {
        match self {
            Self::Sized(sized) => Rect::Sized(sized * rhs),
            Self::Extents(extents) => Rect::Extents(extents * rhs),
        }
    }
}

impl<T, UnitA, UnitB> Div<crate::Scale<T, UnitA, UnitB>> for Rect<T, UnitB>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Rect<T, UnitA>;

    fn div(self, rhs: crate::Scale<T, UnitA, UnitB>) -> Self::Output {
        match self {
            Self::Sized(sized) => Rect::Sized(sized / rhs),
            Self::Extents(extents) => Rect::Extents(extents / rhs),
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
    pub const fn new(origin: Point<T, Unit>, size: Size<T, Unit>) -> Self {
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

impl<T, Unit> Eq for SizedRect<T, Unit> where T: Eq {}

impl<T, Unit> PartialEq for SizedRect<T, Unit>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin) && self.size.eq(&other.size)
    }
}

impl<T, Unit> SizedRect<T, Unit>
where
    T: NumCast + Copy,
{
    /// Attempts to cast `T` to `NewT`. If unsuccessful, None is returned.
    pub fn try_cast<NewT: NumCast + Copy>(&self) -> Option<SizedRect<NewT, Unit>> {
        Some(SizedRect::new(
            self.origin.try_cast()?,
            self.size.try_cast()?,
        ))
    }

    /// Casts `T` to `NewT`.
    ///
    /// # Panics
    ///
    /// Panics if casting fails.
    pub fn cast<NewT: NumCast + Copy>(&self) -> SizedRect<NewT, Unit> {
        self.try_cast().expect("unable to cast")
    }
}

impl<T, Unit> SizedRect<T, Unit>
where
    T: Copy,
{
    /// Returns this value with the new unit. Does not affect the underlying
    /// value.
    pub fn cast_unit<NewUnit>(&self) -> SizedRect<T, NewUnit> {
        SizedRect::new(self.origin.cast_unit(), self.size.cast_unit())
    }
}

impl<T, Unit> Default for SizedRect<T, Unit>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Point::default(), Size::default())
    }
}

impl<T, Unit> Round for SizedRect<T, Unit>
where
    T: Round,
{
    fn round(mut self) -> Self {
        self.origin = self.origin.round();
        self.size = self.size.round();
        self
    }
}

impl<T, UnitA, UnitB> Mul<Scale<T, UnitA, UnitB>> for SizedRect<T, UnitA>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = SizedRect<T, UnitB>;

    fn mul(self, rhs: Scale<T, UnitA, UnitB>) -> Self::Output {
        SizedRect::new(self.origin * rhs, self.size * rhs)
    }
}

impl<T, UnitA, UnitB> Div<crate::Scale<T, UnitA, UnitB>> for SizedRect<T, UnitB>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = SizedRect<T, UnitA>;

    fn div(self, rhs: crate::Scale<T, UnitA, UnitB>) -> Self::Output {
        SizedRect::new(self.origin / rhs, self.size / rhs)
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

impl<T, Unit> ExtentsRect<T, Unit>
where
    T: NumCast + Copy,
{
    /// Attempts to cast `T` to `NewT`. If unsuccessful, None is returned.
    pub fn try_cast<NewT: NumCast + Copy>(&self) -> Option<ExtentsRect<NewT, Unit>> {
        Some(ExtentsRect::new(
            self.origin.try_cast()?,
            self.extent.try_cast()?,
        ))
    }

    /// Casts `T` to `NewT`.
    ///
    /// # Panics
    ///
    /// Panics if casting fails.
    pub fn cast<NewT: NumCast + Copy>(&self) -> ExtentsRect<NewT, Unit> {
        self.try_cast().expect("unable to cast")
    }
}

impl<T, Unit> ExtentsRect<T, Unit>
where
    T: Copy,
{
    /// Returns this value with the new unit. Does not affect the underlying
    /// value.
    pub fn cast_unit<NewUnit>(&self) -> ExtentsRect<T, NewUnit> {
        ExtentsRect::new(self.origin.cast_unit(), self.extent.cast_unit())
    }
}

impl<T, Unit> Eq for ExtentsRect<T, Unit> where T: Eq {}

impl<T, Unit> PartialEq for ExtentsRect<T, Unit>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin) && self.extent.eq(&other.extent)
    }
}

impl<T, Unit> Default for ExtentsRect<T, Unit>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Point::default(), Point::default())
    }
}

impl<T, Unit> Round for ExtentsRect<T, Unit>
where
    T: Round,
{
    fn round(mut self) -> Self {
        self.origin = self.origin.round();
        self.extent = self.extent.round();
        self
    }
}

impl<T, UnitA, UnitB> Mul<Scale<T, UnitA, UnitB>> for ExtentsRect<T, UnitA>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = ExtentsRect<T, UnitB>;

    fn mul(self, rhs: Scale<T, UnitA, UnitB>) -> Self::Output {
        ExtentsRect::new(self.origin * rhs, self.extent * rhs)
    }
}

impl<T, UnitA, UnitB> Div<crate::Scale<T, UnitA, UnitB>> for ExtentsRect<T, UnitB>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = ExtentsRect<T, UnitA>;

    fn div(self, rhs: crate::Scale<T, UnitA, UnitB>) -> Self::Output {
        ExtentsRect::new(self.origin / rhs, self.extent / rhs)
    }
}

/// Functionalitiy that all rectangle types implement
pub trait Rectlike<T, Unit>: Sized
where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
{
    /// Checks to see if this rect is empty. If it is, None is returned. If it
    /// isn't, the rect is returned unmodified.
    fn to_non_empty(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    /// Returns true if the rect doesn't have a positive width and height.
    fn is_empty(&self) -> bool {
        let zero = T::zero();
        !(self.width().get() > zero && self.height().get() > zero)
    }

    /// Returns the area contained by this rectangle.
    fn area(&self) -> Figure<T, Unit> {
        self.width() * self.height()
    }

    /// Returns the width of the rectangle.
    fn width(&self) -> Figure<T, Unit>;
    /// Returns the height of the rectangle.
    fn height(&self) -> Figure<T, Unit>;

    /// Returns the size of the rectangle.
    fn size(&self) -> Size<T, Unit> {
        Size::from_figures(self.width(), self.height())
    }

    /// Returns the origin of the rectangle.
    fn origin(&self) -> Point<T, Unit>;

    /// Returns the center of the rectangle.
    fn center(&self) -> Point<T, Unit> {
        self.origin() + self.size() / (T::one() + T::one())
    }

    /// Returns this rectangle converted to an [`ExtentsRect`].
    fn as_extents(&self) -> ExtentsRect<T, Unit>;
    /// Returns this rectangle converted to a [`SizedRect`].
    fn as_sized(&self) -> SizedRect<T, Unit>;

    /// Returns the intersecting area between the two rectangles. If the
    /// rectangles do not intersect, None is returned.
    fn intersection<R: Rectlike<T, Unit>>(&self, other: &R) -> Option<ExtentsRect<T, Unit>> {
        let r1 = self.as_extents();
        let r2 = other.as_extents();
        ExtentsRect::new(r1.origin.max(&r2.origin), r1.extent.min(&r2.extent)).to_non_empty()
    }

    /// Returns the union of the two rectangles. If both rectangles aren't
    /// empty, the smallest rectangle that both rectangles can fit into will be
    /// returned. If either rectangle is empty, the other rectangle is returned
    /// unmodified.
    fn union<R: Rectlike<T, Unit>>(&self, other: &R) -> Option<ExtentsRect<T, Unit>> {
        match (
            self.as_extents().to_non_empty(),
            other.as_extents().to_non_empty(),
        ) {
            (Some(r1), Some(r2)) => {
                ExtentsRect::new(r1.origin.min(&r2.origin), r1.extent.max(&r2.extent))
                    .to_non_empty()
            }
            (Some(r), None) | (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
}

impl<T, Unit> Rectlike<T, Unit> for Rect<T, Unit>
where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
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

    fn origin(&self) -> Point<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.origin(),
            Rect::Extents(extents) => extents.origin(),
        }
    }
}

impl<T, Unit> Rectlike<T, Unit> for ExtentsRect<T, Unit>
where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
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

    fn origin(&self) -> Point<T, Unit> {
        self.origin
    }
}

impl<T, Unit> Rectlike<T, Unit> for SizedRect<T, Unit>
where
    T: Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
{
    fn width(&self) -> Figure<T, Unit> {
        self.size.width()
    }

    fn height(&self) -> Figure<T, Unit> {
        self.size.height()
    }

    fn as_extents(&self) -> ExtentsRect<T, Unit> {
        ExtentsRect::new(self.origin, self.origin + self.size)
    }

    fn as_sized(&self) -> Self {
        *self
    }

    fn origin(&self) -> Point<T, Unit> {
        self.origin
    }
}

#[test]
fn intersection_tests() {
    let a = SizedRect::<u32, ()>::new(Point::new(10, 20), Size::new(90, 80));
    let b = SizedRect::new(Point::new(50, 50), Size::new(50, 50));
    assert_eq!(
        a.intersection(&b),
        Some(ExtentsRect::new(Point::new(50, 50), Point::new(100, 100)))
    );
}
