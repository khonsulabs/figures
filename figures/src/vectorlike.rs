macro_rules! define_vectorlike {
    ($name:ident, $test_mod_name:ident, $x:ident, $y:ident, $doc:literal) => {
        #[doc = $doc]
        #[allow(missing_docs)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
                f.debug_struct(stringify!($name))
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
            pub fn from_figures($x: crate::Figure<T, Unit>, $y: crate::Figure<T, Unit>) -> Self {
                Self::new($x.get(), $y.get())
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

        impl<T, Unit> crate::Approx<T> for $name<T, Unit>
        where
            T: approx::AbsDiffEq + Copy,
        {
            fn approx_eq(&self, other: &Self) -> bool {
                self.$x().approx_eq(&other.$x()) && self.$y().approx_eq(&other.$y())
            }
        }

        impl<T, Unit> approx::AbsDiffEq for $name<T, Unit>
        where
            T: approx::AbsDiffEq<Epsilon = T> + Copy,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.$x.abs_diff_eq(&other.$x, epsilon) && self.$y.abs_diff_eq(&other.$y, epsilon)
            }

            fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.$x.abs_diff_ne(&other.$x, epsilon) || self.$y.abs_diff_ne(&other.$y, epsilon)
            }
        }

        impl<T, Unit> approx::UlpsEq for $name<T, Unit>
        where
            T: approx::UlpsEq<Epsilon = T> + Copy,
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.$x.ulps_eq(&other.$x, epsilon, max_ulps)
                    && self.$y.ulps_eq(&other.$y, epsilon, max_ulps)
            }

            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.$x.ulps_ne(&other.$x, epsilon, max_ulps)
                    || self.$y.ulps_ne(&other.$y, epsilon, max_ulps)
            }
        }

        impl<T, Unit> approx::RelativeEq for $name<T, Unit>
        where
            T: approx::RelativeEq<Epsilon = T> + Copy,
        {
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.$x.relative_eq(&other.$x, epsilon, max_relative)
                    && self.$y.relative_eq(&other.$y, epsilon, max_relative)
            }

            fn relative_ne(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.$x.relative_ne(&other.$x, epsilon, max_relative)
                    || self.$y.relative_ne(&other.$y, epsilon, max_relative)
            }
        }

        impl<T> crate::Displayable<T> for $name<T, crate::Scaled>
        where
            T: std::ops::Div<T, Output = T> + std::ops::Mul<T, Output = T> + Copy,
        {
            type Pixels = $name<T, crate::Pixels>;
            type Points = $name<T, crate::Points>;
            type Scaled = Self;

            fn to_pixels(&self, scale: &crate::DisplayScale<T>) -> Self::Pixels {
                *self * scale.total
            }

            fn to_points(&self, scale: &crate::DisplayScale<T>) -> Self::Points {
                *self * scale.additional
            }

            fn to_scaled(&self, _scale: &crate::DisplayScale<T>) -> Self::Scaled {
                *self
            }
        }

        impl<T> crate::Displayable<T> for $name<T, crate::Points>
        where
            T: std::ops::Div<T, Output = T> + std::ops::Mul<T, Output = T> + Copy,
        {
            type Pixels = $name<T, crate::Pixels>;
            type Points = Self;
            type Scaled = $name<T, crate::Scaled>;

            fn to_pixels(&self, scale: &crate::DisplayScale<T>) -> Self::Pixels {
                *self * scale.dpi
            }

            fn to_points(&self, _scale: &crate::DisplayScale<T>) -> Self::Points {
                *self
            }

            fn to_scaled(&self, scale: &crate::DisplayScale<T>) -> Self::Scaled {
                *self / scale.additional
            }
        }

        impl<T> crate::Displayable<T> for $name<T, crate::Pixels>
        where
            T: std::ops::Div<T, Output = T> + std::ops::Mul<T, Output = T> + Copy,
        {
            type Pixels = Self;
            type Points = $name<T, crate::Points>;
            type Scaled = $name<T, crate::Scaled>;

            fn to_pixels(&self, _scale: &crate::DisplayScale<T>) -> Self::Pixels {
                *self
            }

            fn to_points(&self, scale: &crate::DisplayScale<T>) -> Self::Points {
                *self / scale.dpi
            }

            fn to_scaled(&self, scale: &crate::DisplayScale<T>) -> Self::Scaled {
                *self / scale.total
            }
        }

        #[cfg(test)]
        mod $test_mod_name {
            use super::*;
            use crate::{
                Approx, Ceil, DisplayScale, Displayable, Floor, Pixels, Points, Round, Scale,
                Scaled,
            };

            #[test]
            fn cast_unit_test() {
                let pixels = $name::<i32, Pixels>::new(1, 2);
                let points: $name<i32, Points> = pixels.cast_unit();
                assert_eq!(pixels.$x, points.$x);
                assert_eq!(pixels.$y, points.$y);
            }

            #[test]
            fn cast_tests() {
                let pixels = $name::<i32, Pixels>::new(256, 255);
                let as_u32 = pixels.cast::<u32>();
                assert_eq!(as_u32.$x, 256);
                assert_eq!(as_u32.$y, 255);
                assert_eq!(pixels.try_cast::<u8>(), None);
            }

            #[test]
            fn signed_tests() {
                let pixels = $name::<i32, Pixels>::new(-256, -255);
                let signum = pixels.signum();
                assert_eq!(signum.x, -1);
                assert_eq!(signum.y, -1);
                let abs = pixels.abs();
                assert_eq!(abs.$x, 256);
                assert_eq!(abs.$y, 255);
                let signum = abs.signum();
                assert_eq!(signum.x, 1);
                assert_eq!(signum.y, 1);
            }

            #[test]
            fn vectorlike_conversions() {
                let original = $name::<i32, Pixels>::new(256, 255);
                let vector = original.to_vector();
                assert_eq!(vector.x, 256);
                assert_eq!(vector.y, 255);
                let point = original.to_point();
                assert_eq!(point.x, 256);
                assert_eq!(point.y, 255);
                let size = original.to_size();
                assert_eq!(size.width, 256);
                assert_eq!(size.height, 255);
            }

            #[test]
            fn math_ops() {
                let one = $name::<i32, Pixels>::new(1, 10);
                let two = one + one;
                assert_eq!(two.$x, 2);
                assert_eq!(two.$y, 20);
                let one_after_sub = two - one;
                assert_eq!(one_after_sub.$x, 1);
                assert_eq!(one_after_sub.$y, 10);
                let neg_one = -one;
                assert_eq!(neg_one.$x, -1);
                assert_eq!(neg_one.$y, -10);
                let four = two * 2;
                assert_eq!(four.$x, 4);
                assert_eq!(four.$y, 40);
                let two_div = four / 2;
                assert_eq!(two_div.$x, 2);
                assert_eq!(two_div.$y, 20);

                let mut value = one;
                value += one;
                assert_eq!(value.$x, 2);
                assert_eq!(value.$y, 20);
                value -= one;
                assert_eq!(value.$x, 1);
                assert_eq!(value.$y, 10);
            }

            #[test]
            fn display_scale_math() {
                let scale = DisplayScale::<u32>::new(Scale::new(2), Scale::new(3));
                let one_scaled = $name::<u32, Scaled>::new(1, 10);
                assert_eq!(one_scaled.to_scaled(&scale), one_scaled);
                let two_points = one_scaled.to_points(&scale);
                assert_eq!(two_points, $name::new(3, 30));
                assert_eq!(two_points.to_points(&scale), two_points);
                let six_pixels = one_scaled.to_pixels(&scale);
                assert_eq!(six_pixels, $name::new(6, 60));
                assert_eq!(six_pixels.to_pixels(&scale), six_pixels);

                assert_eq!(six_pixels.to_points(&scale), two_points);
                assert_eq!(six_pixels.to_scaled(&scale), one_scaled);

                assert_eq!(two_points.to_pixels(&scale), six_pixels);
                assert_eq!(two_points.to_scaled(&scale), one_scaled);
            }

            #[test]
            fn float_ops_test() {
                let one = $name::<f32, Pixels>::new(1., 10.);
                assert!($name::<f32, Pixels>::new(0.5, 9.5).round().approx_eq(&one));
                assert!($name::<f32, Pixels>::new(0.1, 9.1).ceil().approx_eq(&one));
                assert!($name::<f32, Pixels>::new(1.9, 10.9).floor().approx_eq(&one));
            }
        }
    };
}

macro_rules! define_vector_compatibility_ops {
    (
        $name:ident,
        $test_mod_name:ident,
        $x:ident,
        $y:ident,
        $other_name:ident,
        $other_x:ident,
        $other_y:ident
    ) => {
        impl<T, Unit> std::ops::Add<$other_name<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Add<Output = T> + Copy,
        {
            type Output = Self;

            fn add(self, rhs: $other_name<T, Unit>) -> Self::Output {
                Self::new(self.$x + rhs.$other_x, self.$y + rhs.$other_y)
            }
        }

        impl<T, Unit> std::ops::Sub<$other_name<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::Sub<Output = T> + Copy,
        {
            type Output = Self;

            fn sub(self, rhs: $other_name<T, Unit>) -> Self::Output {
                Self::new(self.$x - rhs.$other_x, self.$y - rhs.$other_y)
            }
        }

        impl<T, Unit> std::ops::AddAssign<$other_name<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::AddAssign + Copy,
        {
            fn add_assign(&mut self, rhs: $other_name<T, Unit>) {
                self.$x += rhs.$other_x;
                self.$y += rhs.$other_y;
            }
        }

        impl<T, Unit> std::ops::SubAssign<$other_name<T, Unit>> for $name<T, Unit>
        where
            T: std::ops::SubAssign + Copy,
        {
            fn sub_assign(&mut self, rhs: $other_name<T, Unit>) {
                self.$x -= rhs.$other_x;
                self.$y -= rhs.$other_y;
            }
        }

        impl<T, Unit> PartialEq<$other_name<T, Unit>> for $name<T, Unit>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &$other_name<T, Unit>) -> bool {
                self.$x.eq(&other.$other_x) && self.$y.eq(&other.$other_y)
            }
        }

        #[cfg(test)]
        mod $test_mod_name {
            use super::*;
            use crate::Pixels;

            #[test]
            fn math_ops() {
                let one = $name::<i32, Pixels>::new(1, 10);
                let other_one = $other_name::new(1, 10);
                let two = one + other_one;
                assert_eq!(two.$x, 2);
                assert_eq!(two.$y, 20);
                let one_after_sub = two - other_one;
                assert_eq!(one_after_sub, other_one);

                let mut value = one;
                value += other_one;
                assert_eq!(value.$x, 2);
                assert_eq!(value.$y, 20);
                value -= other_one;
                assert_eq!(value, other_one);
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
    size_tests,
    width,
    height,
    "A measurement of space using width and height."
);
define_vectorlike!(
    Point,
    point_tests,
    x,
    y,
    "A location represented by an x and y value."
);
define_vectorlike!(
    Vector,
    vector_tests,
    x,
    y,
    "A 2d measurement using x and y values."
);

define_vector_compatibility_ops!(Size, size_vector_tests, width, height, Vector, x, y);
define_vector_compatibility_ops!(Point, point_vector_tests, x, y, Vector, x, y);
define_vector_compatibility_ops!(Point, point_size_tests, x, y, Size, width, height);
define_vector_compatibility_ops!(Vector, vector_size_tests, x, y, Size, width, height);

#[test]
fn debug_test() {
    let test = Size::<u32, ()>::new(1, 0);
    assert_eq!(&format!("{:?}", test), "Size { width: 1, height: 0 }");
    let test = Point::<u32, ()>::new(1, 0);
    assert_eq!(&format!("{:?}", test), "Point { x: 1, y: 0 }");
    let test = Vector::<u32, ()>::new(1, 0);
    assert_eq!(&format!("{:?}", test), "Vector { x: 1, y: 0 }");
}

impl<T, Unit> From<Vector<T, Unit>> for Point<T, Unit> {
    fn from(other: Vector<T, Unit>) -> Self {
        Self::new(other.x, other.y)
    }
}

impl<T, Unit> From<Point<T, Unit>> for Vector<T, Unit> {
    fn from(other: Point<T, Unit>) -> Self {
        Self::new(other.x, other.y)
    }
}

impl<T, Unit> From<Vector<T, Unit>> for Size<T, Unit> {
    fn from(other: Vector<T, Unit>) -> Self {
        Self::new(other.x, other.y)
    }
}

impl<T, Unit> From<Size<T, Unit>> for Vector<T, Unit> {
    fn from(other: Size<T, Unit>) -> Self {
        Self::new(other.width, other.height)
    }
}
