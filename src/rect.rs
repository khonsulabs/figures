use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::traits::{IntoSigned, IntoUnsigned};
use crate::{Point, Size};

/// A 2d area expressed as an origin ([`Point`]) and a [`Size`].
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Rect<Unit> {
    /// The origin of the rectangle
    pub origin: Point<Unit>,
    /// The size of the rectangle.
    pub size: Size<Unit>,
}

impl<Unit> Rect<Unit> {
    /// Returns a new rectangle.
    pub const fn new(origin: Point<Unit>, size: Size<Unit>) -> Self {
        Self { origin, size }
    }

    /// Returns a new rectangle using the given points to form the top-left and
    /// bottom-right of the rectangle.
    ///
    /// The order of the parameters does not matter. The minimum values will
    /// form the top-left and the maximum values will form the bottom-right.
    pub fn from_extents(p1: Point<Unit>, p2: Point<Unit>) -> Self
    where
        Unit: crate::Unit,
    {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        Self {
            origin: Point { x: min_x, y: min_y },
            size: Size {
                width: max_x - min_x,
                height: max_y - min_y,
            },
        }
    }

    /// Returns a rectangle that has been inset by `amount` on all sides.
    pub fn inset(mut self, amount: impl Into<Unit>) -> Self
    where
        Unit: Add<Unit, Output = Unit> + AddAssign<Unit> + SubAssign<Unit> + Copy,
    {
        let amount = amount.into();
        let double_amount = amount + amount;
        self.origin.x += amount;
        self.origin.y += amount;
        self.size.width -= double_amount;
        self.size.height -= double_amount;
        self
    }

    /// Converts the contents of this point to `NewUnit` using [`From`].
    pub fn cast<NewUnit>(self) -> Rect<NewUnit>
    where
        NewUnit: From<Unit>,
    {
        Rect {
            origin: self.origin.cast(),
            size: self.size.cast(),
        }
    }

    /// Converts the contents of this rect to `NewUnit` using [`TryFrom`].
    ///
    /// # Errors
    ///
    /// Returns `<NewUnit as TryFrom>::Error` when the inner type cannot be
    /// converted. For this crate's types, this genenerally will be
    pub fn try_cast<NewUnit>(self) -> Result<Rect<NewUnit>, NewUnit::Error>
    where
        NewUnit: TryFrom<Unit>,
    {
        Ok(Rect {
            origin: self.origin.try_cast()?,
            size: self.size.try_cast()?,
        })
    }

    /// Returns true if this rect contains `point`.
    pub fn contains(&self, point: Point<Unit>) -> bool
    where
        Unit: crate::Unit,
    {
        let (p1, p2) = self.extents();
        p1.x <= point.x && p1.y <= point.y && p2.x > point.x && p2.y > point.y
    }

    /// Returns true if the areas of `self` and `other` overlap.
    ///
    /// This function does not return true if the edges touch but do not overlap.
    ///
    /// ```rust
    /// use figures::{Point, Rect, Size};
    ///
    /// let a: Rect<i32> = Rect::new(Point::new(1, 1), Size::new(2, 2));
    /// let b = Rect::new(Point::new(2, 2), Size::new(1, 1));
    /// assert!(a.intersects(&b));
    /// let c = Rect::new(Point::new(3, 1), Size::new(1, 1));
    /// assert!(!a.intersects(&c));
    /// ```
    pub fn intersects(&self, other: &Self) -> bool
    where
        Unit: Add<Output = Unit> + Ord + Copy,
    {
        let (
            Point {
                x: r1_left,
                y: r1_top,
            },
            Point {
                x: r1_right,
                y: r1_bottom,
            },
        ) = self.extents();
        let (
            Point {
                x: r2_left,
                y: r2_top,
            },
            Point {
                x: r2_right,
                y: r2_bottom,
            },
        ) = other.extents();
        !(r1_right <= r2_left || r2_right <= r1_left || r1_bottom <= r2_top || r1_top >= r2_bottom)
    }

    /// Returns the overlapping rectangle of `self` and `other`. If the
    /// rectangles do not overlap, None will be returned.
    ///
    /// ```rust
    /// use figures::{Point, Rect, Size};
    ///
    /// let a: Rect<i32> = Rect::new(Point::new(1, 1), Size::new(3, 3));
    /// let b = Rect::new(Point::new(2, 2), Size::new(3, 3));
    /// assert_eq!(
    ///     a.intersection(&b),
    ///     Some(Rect::new(Point::new(2, 2), Size::new(2, 2)))
    /// );
    /// let c = Rect::new(Point::new(4, 1), Size::new(1, 1));
    /// assert_eq!(a.intersection(&c), None);
    /// ```
    pub fn intersection(&self, other: &Self) -> Option<Rect<Unit>>
    where
        Unit: crate::Unit,
    {
        let (a1, a2) = self.extents();
        let (b1, b2) = other.extents();
        let x1 = a1.x.max(b1.x);
        let x2 = a2.x.min(b2.x);
        if x2 > x1 {
            let y1 = a1.y.max(b1.y);
            let y2 = a2.y.min(b2.y);
            if y2 > y1 {
                return Some(Rect::from_extents(Point::new(x1, y1), Point::new(x2, y2)));
            }
        }
        None
    }

    /// Returns the non-origin point.
    pub fn extent(&self) -> Point<Unit>
    where
        Unit: crate::Unit,
    {
        self.origin + self.size
    }
}

impl<Unit> Rect<Unit>
where
    Unit: Add<Output = Unit> + Ord + Copy,
{
    /// Returns the top-left and bottom-right points of this rectangle.
    ///
    /// The first point returned will always be the top-right point, even if the size of the rectangle is negative.
    pub fn extents(&self) -> (Point<Unit>, Point<Unit>) {
        let extent = self.origin + self.size;
        (
            Point::new(self.origin.x.min(extent.x), self.origin.y.min(extent.y)),
            Point::new(self.origin.x.max(extent.x), self.origin.y.max(extent.y)),
        )
    }
}

impl<Unit> Default for Rect<Unit>
where
    Unit: Default,
{
    fn default() -> Self {
        Self {
            origin: Point::default(),
            size: Size::default(),
        }
    }
}

impl<Unit> IntoUnsigned for Rect<Unit>
where
    Unit: IntoUnsigned,
{
    type Unsigned = Rect<Unit::Unsigned>;

    fn into_unsigned(self) -> Self::Unsigned {
        Rect {
            origin: self.origin.into_unsigned(),
            size: self.size.into_unsigned(),
        }
    }
}

impl<Unit> IntoSigned for Rect<Unit>
where
    Unit: IntoSigned,
{
    type Signed = Rect<Unit::Signed>;

    fn into_signed(self) -> Self::Signed {
        Rect {
            origin: self.origin.into_signed(),
            size: self.size.into_signed(),
        }
    }
}

impl<Unit> From<Size<Unit>> for Rect<Unit>
where
    Unit: Default,
{
    fn from(size: Size<Unit>) -> Self {
        Self::new(Point::default(), size)
    }
}

impl<Unit> Add<Point<Unit>> for Rect<Unit>
where
    Unit: Add<Output = Unit>,
{
    type Output = Self;

    fn add(self, rhs: Point<Unit>) -> Self::Output {
        Self::new(self.origin + rhs, self.size)
    }
}

impl<Unit> Sub<Point<Unit>> for Rect<Unit>
where
    Unit: Sub<Output = Unit>,
{
    type Output = Self;

    fn sub(self, rhs: Point<Unit>) -> Self::Output {
        Self::new(self.origin - rhs, self.size)
    }
}

#[test]
fn intersection() {
    assert_eq!(
        Rect::<i32>::new(Point::new(1, 1,), Size::new(3, 3))
            .intersection(&Rect::new(Point::new(2, 2,), Size::new(3, 3))),
        Some(Rect::new(Point::new(2, 2,), Size::new(2, 2)))
    );
}
