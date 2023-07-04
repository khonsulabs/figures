use std::ops::{Add, Sub};

use crate::{Point, Size};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Rect<Unit> {
    pub origin: Point<Unit>,
    pub size: Size<Unit>,
}

impl<Unit> Rect<Unit> {
    pub const fn new(origin: Point<Unit>, size: Size<Unit>) -> Self {
        Self { origin, size }
    }

    pub fn from_extents(p1: Point<Unit>, p2: Point<Unit>) -> Self
    where
        Unit: Copy + Ord + Sub<Output = Unit>,
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

    pub fn into_u32(self) -> Rect<u32>
    where
        Point<Unit>: Into<Point<u32>>,
        Size<Unit>: Into<Size<u32>>,
    {
        Rect {
            origin: self.origin.into(),
            size: self.size.into(),
        }
    }

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

impl<Unit> Rect<Unit>
where
    Unit: Add<Output = Unit> + Ord + Copy,
{
    pub fn extents(&self) -> (Point<Unit>, Point<Unit>) {
        let extent = self.origin + self.size;
        (
            Point::new(self.origin.x.min(extent.x), self.origin.y.min(extent.y)),
            Point::new(self.origin.x.max(extent.x), self.origin.y.max(extent.y)),
        )
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
