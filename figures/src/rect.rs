use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{NumCast, One, Zero};

use crate::{
    Ceil, DisplayScale, Displayable, Figure, Floor, Pixels, Point, Points, Round, Scale, Scaled,
    Size, Vector, Vectorlike,
};

/// A 2d rectangle. This type may internally be represented with a [`SizedRect`]
/// or an [`ExtentsRect`]. All rect types implement [`Rectlike`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rect<T, Unit> {
    /// A [`SizedRect`].
    Sized(SizedRect<T, Unit>),
    /// An [`ExtentsRect`].
    Extents(ExtentsRect<T, Unit>),
}

impl<T, Unit> Debug for Rect<T, Unit>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rect::Sized(sized) => sized.fmt(f),
            Rect::Extents(extents) => extents.fmt(f),
        }
    }
}

impl<T, Unit> From<SizedRect<T, Unit>> for Rect<T, Unit> {
    fn from(rect: SizedRect<T, Unit>) -> Self {
        Self::Sized(rect)
    }
}

impl<T, Unit> From<ExtentsRect<T, Unit>> for Rect<T, Unit> {
    fn from(rect: ExtentsRect<T, Unit>) -> Self {
        Self::Extents(rect)
    }
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

impl<T, Unit> Ceil for Rect<T, Unit>
where
    T: Ceil,
{
    fn ceil(self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.ceil()),
            Self::Extents(extents) => Self::Extents(extents.ceil()),
        }
    }
}

impl<T, Unit> Floor for Rect<T, Unit>
where
    T: Floor,
{
    fn floor(self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.floor()),
            Self::Extents(extents) => Self::Extents(extents.floor()),
        }
    }
}

impl<T, Unit> Rect<T, Unit>
where
    T: Ceil + Floor,
{
    /// Returns a new rectangle that rounds the origin down using `floor` and
    /// rounds the extent/size out using `ceil`.
    pub fn round_out(self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.round_out()),
            Self::Extents(extents) => Self::Extents(extents.round_out()),
        }
    }

    /// Returns a new rectangle that rounds the origin up using `ceil` and
    /// rounds the extent/size in using `floor`.
    pub fn round_in(self) -> Self {
        match self {
            Self::Sized(sized) => Self::Sized(sized.round_in()),
            Self::Extents(extents) => Self::Extents(extents.round_in()),
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

impl<T, Unit> crate::Approx<T> for Rect<T, Unit>
where
    T: approx::AbsDiffEq
        + Zero
        + PartialOrd
        + Mul<T, Output = T>
        + Sub<T, Output = T>
        + One
        + Add<T, Output = T>
        + Div<T, Output = T>
        + Copy,
{
    fn approx_eq(&self, other: &Self) -> bool {
        match self {
            Self::Sized(sized) => sized.approx_eq(&other.as_sized()),
            Self::Extents(extents) => extents.approx_eq(&other.as_extents()),
        }
    }
}
impl<T> Displayable<T> for Rect<T, Scaled>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Rect<T, Pixels>;
    type Points = Rect<T, Points>;
    type Scaled = Self;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_pixels(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_pixels(scale)),
        }
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_points(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_points(scale)),
        }
    }

    fn to_scaled(&self, _scale: &DisplayScale<T>) -> Self::Scaled {
        *self
    }
}

impl<T> Displayable<T> for Rect<T, Points>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Rect<T, Pixels>;
    type Points = Self;
    type Scaled = Rect<T, Scaled>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_pixels(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_pixels(scale)),
        }
    }

    fn to_points(&self, _scale: &DisplayScale<T>) -> Self::Points {
        *self
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_scaled(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_scaled(scale)),
        }
    }
}

impl<T> Displayable<T> for Rect<T, Pixels>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Self;
    type Points = Rect<T, Points>;
    type Scaled = Rect<T, Scaled>;

    fn to_pixels(&self, _scale: &DisplayScale<T>) -> Self::Pixels {
        *self
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_points(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_points(scale)),
        }
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        match self {
            Self::Sized(sized) => Rect::Sized(sized.to_scaled(scale)),
            Self::Extents(extents) => Rect::Extents(extents.to_scaled(scale)),
        }
    }
}

/// A rectangle that uses a [`Point`] and a [`Size`] for representation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizedRect<T, Unit> {
    /// The origin of the rectangle.
    pub origin: Point<T, Unit>,
    /// The size of the rectangle.
    pub size: Size<T, Unit>,
}

impl<T, Unit> Debug for SizedRect<T, Unit>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SizedRect")
            .field("origin", &self.origin)
            .field("size", &self.size)
            .finish()
    }
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

impl<T, Unit> Ceil for SizedRect<T, Unit>
where
    T: Ceil,
{
    fn ceil(mut self) -> Self {
        self.origin = self.origin.ceil();
        self.size = self.size.ceil();
        self
    }
}

impl<T, Unit> Floor for SizedRect<T, Unit>
where
    T: Floor,
{
    fn floor(mut self) -> Self {
        self.origin = self.origin.floor();
        self.size = self.size.floor();
        self
    }
}

impl<T, Unit> SizedRect<T, Unit>
where
    T: Ceil + Floor,
{
    /// Returns a new rectangle that rounds the origin down using `floor` and
    /// rounds the size out using `ceil`.
    pub fn round_out(mut self) -> Self {
        self.origin = self.origin.floor();
        self.size = self.size.ceil();
        self
    }

    /// Returns a new rectangle that rounds the origin up using `ceil` and
    /// rounds the size in using `floor`.
    pub fn round_in(mut self) -> Self {
        self.origin = self.origin.ceil();
        self.size = self.size.floor();
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

impl<T, Unit> crate::Approx<T> for SizedRect<T, Unit>
where
    T: approx::AbsDiffEq + Copy,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.origin.approx_eq(&other.origin) && self.size.approx_eq(&other.size)
    }
}

impl<T> Displayable<T> for SizedRect<T, Scaled>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = SizedRect<T, Pixels>;
    type Points = SizedRect<T, Points>;
    type Scaled = Self;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.scaled
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self / scale.between
    }

    fn to_scaled(&self, _scale: &DisplayScale<T>) -> Self::Scaled {
        *self
    }
}

impl<T> Displayable<T> for SizedRect<T, Points>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = SizedRect<T, Pixels>;
    type Points = Self;
    type Scaled = SizedRect<T, Scaled>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.points
    }

    fn to_points(&self, _scale: &DisplayScale<T>) -> Self::Points {
        *self
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.between
    }
}

impl<T> Displayable<T> for SizedRect<T, Pixels>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Self;
    type Points = SizedRect<T, Points>;
    type Scaled = SizedRect<T, Scaled>;

    fn to_pixels(&self, _scale: &DisplayScale<T>) -> Self::Pixels {
        *self
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self * scale.points
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.scaled
    }
}

/// A rectangle that uses two [`Point`]s for representation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtentsRect<T, Unit> {
    /// The origin of the rectangle.
    pub origin: Point<T, Unit>,
    /// The non-origin point of the rectangle.
    pub extent: Point<T, Unit>,
}

impl<T, Unit> Debug for ExtentsRect<T, Unit>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SizedRect")
            .field("origin", &self.origin)
            .field("extent", &self.extent)
            .finish()
    }
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

impl<T, Unit> Ceil for ExtentsRect<T, Unit>
where
    T: Ceil,
{
    fn ceil(mut self) -> Self {
        self.origin = self.origin.ceil();
        self.extent = self.extent.ceil();
        self
    }
}

impl<T, Unit> Floor for ExtentsRect<T, Unit>
where
    T: Floor,
{
    fn floor(mut self) -> Self {
        self.origin = self.origin.floor();
        self.extent = self.extent.floor();
        self
    }
}

impl<T, Unit> ExtentsRect<T, Unit>
where
    T: Ceil + Floor,
{
    /// Returns a new rectangle that rounds the origin down using `floor` and
    /// rounds the extent out using `ceil`.
    pub fn round_out(mut self) -> Self {
        self.origin = self.origin.floor();
        self.extent = self.extent.ceil();
        self
    }

    /// Returns a new rectangle that rounds the origin up using `ceil` and
    /// rounds the extent in using `floor`.
    pub fn round_in(mut self) -> Self {
        self.origin = self.origin.ceil();
        self.extent = self.extent.floor();
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

impl<T, Unit> crate::Approx<T> for ExtentsRect<T, Unit>
where
    T: approx::AbsDiffEq + Copy,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.origin.approx_eq(&other.origin) && self.extent.approx_eq(&other.extent)
    }
}

impl<T> Displayable<T> for ExtentsRect<T, Scaled>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = ExtentsRect<T, Pixels>;
    type Points = ExtentsRect<T, Points>;
    type Scaled = Self;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.scaled
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self / scale.between
    }

    fn to_scaled(&self, _scale: &DisplayScale<T>) -> Self::Scaled {
        *self
    }
}

impl<T> Displayable<T> for ExtentsRect<T, Points>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = ExtentsRect<T, Pixels>;
    type Points = Self;
    type Scaled = ExtentsRect<T, Scaled>;

    fn to_pixels(&self, scale: &DisplayScale<T>) -> Self::Pixels {
        *self / scale.points
    }

    fn to_points(&self, _scale: &DisplayScale<T>) -> Self::Points {
        *self
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.between
    }
}

impl<T> Displayable<T> for ExtentsRect<T, Pixels>
where
    T: Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Pixels = Self;
    type Points = ExtentsRect<T, Points>;
    type Scaled = ExtentsRect<T, Scaled>;

    fn to_pixels(&self, _scale: &DisplayScale<T>) -> Self::Pixels {
        *self
    }

    fn to_points(&self, scale: &DisplayScale<T>) -> Self::Points {
        *self * scale.points
    }

    fn to_scaled(&self, scale: &DisplayScale<T>) -> Self::Scaled {
        *self * scale.scaled
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
    /// Returns this rectangle as a `Rect`. The rectangle's underlying data will
    /// be unchanged by this operation.
    fn as_rect(&self) -> Rect<T, Unit>;
    /// Returns this rectangle converted to an [`ExtentsRect`].
    fn as_extents(&self) -> ExtentsRect<T, Unit>;
    /// Returns this rectangle converted to a [`SizedRect`].
    fn as_sized(&self) -> SizedRect<T, Unit>;

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

    /// Returns true if `point` is within this rectangle.
    fn contains(&self, point: Point<T, Unit>) -> bool {
        let extents = self.as_extents();
        extents.origin.x <= point.x
            && extents.origin.y <= point.y
            && extents.extent.x >= point.x
            && extents.extent.y >= point.y
    }

    /// Moves this rectangle by the vector provided.
    fn translate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self;

    /// Increases the size of this rectangle by the vector provided. The rectangle will grow around its center.
    fn inflate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self;

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
    fn as_rect(&self) -> Rect<T, Unit> {
        *self
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

    fn origin(&self) -> Point<T, Unit> {
        match self {
            Rect::Sized(sized) => sized.origin(),
            Rect::Extents(extents) => extents.origin(),
        }
    }

    fn translate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        match self {
            Rect::Sized(sized) => Rect::Sized(sized.translate(by)),
            Rect::Extents(extents) => Rect::Extents(extents.translate(by)),
        }
    }

    fn inflate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        match self {
            Rect::Sized(sized) => Rect::Sized(sized.inflate(by)),
            Rect::Extents(extents) => Rect::Extents(extents.inflate(by)),
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
    fn as_rect(&self) -> Rect<T, Unit> {
        Rect::Extents(*self)
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

    fn width(&self) -> Figure<T, Unit> {
        self.extent.x() - self.origin.x()
    }

    fn height(&self) -> Figure<T, Unit> {
        self.extent.y() - self.origin.y()
    }

    fn origin(&self) -> Point<T, Unit> {
        self.origin
    }

    fn translate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        let by = by.into();
        Self {
            origin: self.origin + by,
            extent: self.extent + by,
        }
    }

    fn inflate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        let by = by.into() / (T::one() + T::one());
        Self {
            origin: self.origin - by,
            extent: self.extent + by,
        }
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
    fn as_rect(&self) -> Rect<T, Unit> {
        Rect::Sized(*self)
    }

    fn as_extents(&self) -> ExtentsRect<T, Unit> {
        ExtentsRect::new(self.origin, self.origin + self.size)
    }

    fn as_sized(&self) -> Self {
        *self
    }

    fn width(&self) -> Figure<T, Unit> {
        self.size.width()
    }

    fn height(&self) -> Figure<T, Unit> {
        self.size.height()
    }

    fn origin(&self) -> Point<T, Unit> {
        self.origin
    }

    fn translate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        let by = by.into();
        Self {
            origin: self.origin + by,
            size: self.size,
        }
    }

    fn inflate<V: Into<Vector<T, Unit>>>(&self, by: V) -> Self {
        let by = by.into() / (T::one() + T::one());
        Self {
            origin: self.origin - by,
            size: self.size + by,
        }
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

#[test]
fn contains_test() {
    let a = SizedRect::<u32, ()>::new(Point::new(10, 20), Size::new(30, 40));
    assert!(a.contains(Point::new(10, 20)));
    assert!(a.contains(Point::new(11, 20)));
    assert!(a.contains(Point::new(10, 21)));
    assert!(a.contains(Point::new(40, 60)));
    assert!(a.contains(Point::new(39, 60)));
    assert!(a.contains(Point::new(40, 59)));
    assert!(!a.contains(Point::new(9, 20)));
    assert!(!a.contains(Point::new(10, 19)));
    assert!(!a.contains(Point::new(41, 20)));
    assert!(!a.contains(Point::new(40, 19)));
    assert!(!a.contains(Point::new(9, 60)));
    assert!(!a.contains(Point::new(10, 61)));
    assert!(!a.contains(Point::new(41, 60)));
    assert!(!a.contains(Point::new(40, 61)));
}
