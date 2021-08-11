macro_rules! define_vectorlike {
    ($name:ident, $x:ident, $y:ident, $doc:literal) => {
        #[doc = $doc]
        #[allow(missing_docs)]
        pub struct $name<T, Unit> {
            pub $x: T,
            pub $y: T,
            _unit: std::marker::PhantomData<Unit>,
        }

        impl<T, Unit> Copy for $name<T, Unit> where T: Copy {}

        impl<T, Unit> Clone for $name<T, Unit>
        where
            T: Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    $x: self.$x.clone(),
                    $y: self.$y.clone(),
                    _unit: std::marker::PhantomData::default(),
                }
            }
        }

        impl<T, Unit> std::fmt::Debug for $name<T, Unit>
        where
            T: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(&format!(
                    "{}<{}, {}>",
                    stringify!($name),
                    std::any::type_name::<T>(),
                    std::any::type_name::<Unit>(),
                ))
                .field(stringify!($x), &self.$x)
                .field(stringify!($y), &self.$y)
                .finish()
            }
        }

        impl<T, Unit> $name<T, Unit> {
            ///Returns a new `
            #[doc = stringify!($name)]
            ///`.
            pub const fn new($x: T, $y: T) -> Self {
                Self {
                    $x,
                    $y,
                    _unit: std::marker::PhantomData,
                }
            }
        }

        impl<T, Unit> $name<T, Unit>
        where
            T: Copy,
        {
            ///Returns a new `
            #[doc = stringify!($name)]
            ///`.
            pub fn from_figures(
                $x: impl Into<crate::Figure<T, Unit>>,
                $y: impl Into<crate::Figure<T, Unit>>,
            ) -> Self {
                Self::new($x.into().get(), $y.into().get())
            }

            ///Returns the
            #[doc = stringify!($x)]
            /// component.
            pub fn $x(&self) -> crate::Figure<T, Unit> {
                crate::Figure::from(self.$x)
            }

            ///Returns the
            #[doc = stringify!($y)]
            /// component.
            pub fn $y(&self) -> crate::Figure<T, Unit> {
                crate::Figure::from(self.$y)
            }

            /// Returns this value with the new unit. Does not affect the underlying
            /// value.
            pub fn cast_unit<NewUnit>(&self) -> $name<T, NewUnit> {
                $name::new(self.$x, self.$y)
            }
        }

        impl<T, Unit> $name<T, Unit>
        where
            T: num_traits::NumCast + Copy,
        {
            /// Attempts to convert from `T` to `Output` using
            /// [`NumCast`](num_traits::NumCast). Returns None if the value
            /// can't be converted.
            pub fn try_cast<Output: num_traits::NumCast + Copy>(
                self,
            ) -> Option<$name<Output, Unit>> {
                match (Output::from(self.$x), Output::from(self.$y)) {
                    (Some($x), Some($y)) => Some($name::new($x, $y)),
                    _ => None,
                }
            }

            /// Converts from `T` to `Output`.
            ///
            /// # Panics
            ///
            /// Panics if `Self::try_cast()` returns `None`.
            pub fn cast<Output: num_traits::NumCast + Copy>(self) -> $name<Output, Unit> {
                self.try_cast().expect("could not cast")
            }
        }

        impl<T, Unit> $name<T, Unit>
        where
            T: num_traits::Signed + Copy,
        {
            /// Returns a new instance with the absolute value of each
            /// component.
            pub fn abs(self) -> Self {
                Self::new(self.$x.abs(), self.$y.abs())
            }

            /// Returns a `Vector` with the result of `signum()` called on each
            /// component.
            pub fn signum(&self) -> crate::Vector<T, Unit> {
                crate::Vector::new(self.$x.signum(), self.$y.signum())
            }
        }

        impl<T, Unit> $name<T, Unit>
        where
            T: std::cmp::PartialOrd + Copy,
        {
            ///Returns a new `
            #[doc = stringify!($name)]
            ///` with the smaller value of each component.
            pub fn min(&self, rhs: &Self) -> Self {
                Self::from_figures(self.$x().min(rhs.$x()), self.$y().min(rhs.$y()))
            }

            ///Returns a new `
            #[doc = stringify!($name)]
            ///` with the larger value of each component.
            pub fn max(&self, rhs: &Self) -> Self {
                Self::from_figures(self.$x().max(rhs.$x()), self.$y().max(rhs.$y()))
            }
        }

        impl<T, Unit> Default for $name<T, Unit>
        where
            T: Default,
        {
            fn default() -> Self {
                Self {
                    $x: T::default(),
                    $y: T::default(),
                    _unit: std::marker::PhantomData::default(),
                }
            }
        }

        impl<T, Unit> crate::Vectorlike<T, Unit> for $name<T, Unit>
        where
            T: Copy,
        {
            fn to_vector(&self) -> crate::Vector<T, Unit> {
                crate::Vector::new(self.$x, self.$y)
            }

            fn to_size(&self) -> crate::Size<T, Unit> {
                crate::Size::new(self.$x, self.$y)
            }

            fn to_point(&self) -> crate::Point<T, Unit> {
                crate::Point::new(self.$x, self.$y)
            }
        }

        impl<T, Unit> std::ops::Add for $name<T, Unit>
        where
            T: std::ops::Add<Output = T> + Copy,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.$x + rhs.$x, self.$y + rhs.$y)
            }
        }

        impl<T, Unit> std::ops::Sub for $name<T, Unit>
        where
            T: std::ops::Sub<Output = T> + Copy,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.$x - rhs.$x, self.$y - rhs.$y)
            }
        }

        impl<T, Unit> std::ops::AddAssign for $name<T, Unit>
        where
            T: std::ops::AddAssign + Copy,
        {
            fn add_assign(&mut self, rhs: Self) {
                self.$x += rhs.$x;
                self.$y += rhs.$y;
            }
        }

        impl<T, Unit> std::ops::SubAssign for $name<T, Unit>
        where
            T: std::ops::SubAssign + Copy,
        {
            fn sub_assign(&mut self, rhs: Self) {
                self.$x -= rhs.$x;
                self.$y -= rhs.$y;
            }
        }

        impl<T, Unit> std::ops::Neg for $name<T, Unit>
        where
            T: std::ops::Neg<Output = T> + Copy,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.$x, -self.$y)
            }
        }

        impl<T, Unit> Eq for $name<T, Unit> where T: Eq {}

        impl<T, Unit> PartialEq for $name<T, Unit>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.$x.eq(&other.$x) && self.$y.eq(&other.$y)
            }
        }

        impl<T, UnitA, UnitB> std::ops::Mul<crate::Scale<T, UnitA, UnitB>> for $name<T, UnitA>
        where
            T: std::ops::Mul<T, Output = T> + Copy,
        {
            type Output = $name<T, UnitB>;

            fn mul(self, rhs: crate::Scale<T, UnitA, UnitB>) -> Self::Output {
                $name::new(self.$x * rhs.get(), self.$y * rhs.get())
            }
        }

        impl<T, UnitA, UnitB> std::ops::Div<crate::Scale<T, UnitA, UnitB>> for $name<T, UnitB>
        where
            T: std::ops::Div<T, Output = T> + Copy,
        {
            type Output = $name<T, UnitA>;

            fn div(self, rhs: crate::Scale<T, UnitA, UnitB>) -> Self::Output {
                $name::new(self.$x / rhs.get(), self.$y / rhs.get())
            }
        }
        impl<T, Unit> std::ops::Mul<T> for $name<T, Unit>
        where
            T: std::ops::Mul<T, Output = T> + Copy,
        {
            type Output = $name<T, Unit>;

            fn mul(self, rhs: T) -> Self::Output {
                $name::new(self.$x * rhs, self.$y * rhs)
            }
        }

        impl<T, Unit> std::ops::Div<T> for $name<T, Unit>
        where
            T: std::ops::Div<T, Output = T> + Copy,
        {
            type Output = $name<T, Unit>;

            fn div(self, rhs: T) -> Self::Output {
                $name::new(self.$x / rhs, self.$y / rhs)
            }
        }

        impl<T, Unit> crate::num::Round for $name<T, Unit>
        where
            T: crate::num::Round,
        {
            fn round(mut self) -> Self {
                self.$x = self.$x.round();
                self.$y = self.$y.round();
                self
            }
        }

        impl<T, Unit> crate::num::Ceil for $name<T, Unit>
        where
            T: crate::num::Ceil,
        {
            fn ceil(mut self) -> Self {
                self.$x = self.$x.ceil();
                self.$y = self.$y.ceil();
                self
            }
        }

        impl<T, Unit> crate::num::Floor for $name<T, Unit>
        where
            T: crate::num::Floor,
        {
            fn floor(mut self) -> Self {
                self.$x = self.$x.floor();
                self.$y = self.$y.floor();
                self
            }
        }
    };
}

macro_rules! define_vector_compatibility_ops {
    ($name:ident, $x:ident, $y:ident) => {
        impl<T, Unit> std::ops::Add<Vector<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Add<Output = T> + Copy,
        {
            type Output = Self;

            fn add(self, rhs: Vector<T, Unit>) -> Self::Output {
                Self::new(self.$x + rhs.x, self.$y + rhs.y)
            }
        }

        impl<T, Unit> std::ops::Sub<Vector<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Sub<Output = T> + Copy,
        {
            type Output = Self;

            fn sub(self, rhs: Vector<T, Unit>) -> Self::Output {
                Self::new(self.$x - rhs.x, self.$y - rhs.y)
            }
        }

        impl<T, Unit> std::ops::AddAssign<Vector<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::AddAssign + Copy,
        {
            fn add_assign(&mut self, rhs: Vector<T, Unit>) {
                self.$x += rhs.x;
                self.$y += rhs.y;
            }
        }

        impl<T, Unit> std::ops::SubAssign<Vector<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::SubAssign + Copy,
        {
            fn sub_assign(&mut self, rhs: Vector<T, Unit>) {
                self.$x -= rhs.x;
                self.$y -= rhs.y;
            }
        }

        impl<T, Unit> PartialEq<Vector<T, Unit>> for $name<T, Unit>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Vector<T, Unit>) -> bool {
                self.$x.eq(&other.x) && self.$y.eq(&other.y)
            }
        }
    };
}

macro_rules! define_size_compatibility_ops {
    ($name:ident, $x:ident, $y:ident) => {
        impl<T, Unit> std::ops::Add<Size<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Add<Output = T> + Copy,
        {
            type Output = Self;

            fn add(self, rhs: Size<T, Unit>) -> Self::Output {
                Self::new(self.$x + rhs.width, self.$y + rhs.height)
            }
        }

        impl<T, Unit> std::ops::Sub<Size<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Sub<Output = T> + Copy,
        {
            type Output = Self;

            fn sub(self, rhs: Size<T, Unit>) -> Self::Output {
                Self::new(self.$x - rhs.width, self.$y - rhs.height)
            }
        }

        impl<T, Unit> std::ops::AddAssign<Size<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::AddAssign + Copy,
        {
            fn add_assign(&mut self, rhs: Size<T, Unit>) {
                self.$x += rhs.width;
                self.$y += rhs.height;
            }
        }

        impl<T, Unit> std::ops::SubAssign<Size<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::SubAssign + Copy,
        {
            fn sub_assign(&mut self, rhs: Size<T, Unit>) {
                self.$x -= rhs.width;
                self.$y -= rhs.height;
            }
        }

        impl<T, Unit> PartialEq<Size<T, Unit>> for $name<T, Unit>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Size<T, Unit>) -> bool {
                self.$x.eq(&other.width) && self.$y.eq(&other.height)
            }
        }
    };
}

/// Methods that enable converting between 2d types that have two components.
pub trait Vectorlike<T, Unit> {
    /// Returns `self` as a `Vector`.
    fn to_vector(&self) -> Vector<T, Unit>;
    /// Returns `self` as a `Size`.
    fn to_size(&self) -> Size<T, Unit>;
    /// Returns `self` as a `Point`.
    fn to_point(&self) -> Point<T, Unit>;
}

define_vectorlike!(
    Size,
    width,
    height,
    "A measurement of space using width and height."
);
define_vectorlike!(Point, x, y, "A location represented by an x and y value.");
define_vectorlike!(Vector, x, y, "A 2d measurement using x and y values.");

define_vector_compatibility_ops!(Size, width, height);
define_vector_compatibility_ops!(Point, x, y);
define_size_compatibility_ops!(Point, x, y);
define_size_compatibility_ops!(Vector, x, y);

#[test]
fn debug_test() {
    let test = Size::<u32, ()>::new(1, 0);
    assert_eq!(
        &format!("{:?}", test),
        "Size<u32, ()> { width: 1, height: 0 }"
    );
    let test = Point::<u32, ()>::new(1, 0);
    assert_eq!(&format!("{:?}", test), "Point<u32, ()> { x: 1, y: 0 }");
    let test = Vector::<u32, ()>::new(1, 0);
    assert_eq!(&format!("{:?}", test), "Vector<u32, ()> { x: 1, y: 0 }");
}
