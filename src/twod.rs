macro_rules! impl_2d_math {
    ($type:ident, $x:ident, $y:ident) => {
        mod twodmath {
            use std::ops::Neg;

            use super::$type;
            use crate::traits::{
                FloatConversion, FromComponents, IntoComponents, IntoSigned, IntoUnsigned, Ranged,
                Round, ScreenScale, Zero, Abs, Pow,
            };
            use crate::units::{Lp, Px, UPx};

            impl<Unit> Zero for $type<Unit>
            where
                Unit: Zero,
            {
                const ZERO: Self = Self::new(Unit::ZERO, Unit::ZERO);

                fn is_zero(&self) -> bool {
                    self.$x.is_zero() && self.$y.is_zero()
                }
            }

            impl<Unit> Pow for $type<Unit>
            where
                Unit: Pow,
            {
                fn pow(&self, exp: u32) -> Self {
                    Self {
                        $x: self.$x.pow(exp),
                        $y: self.$y.pow(exp),
                    }
                }
            }

            impl<Unit> Abs for $type<Unit>
            where
                Unit: Abs,
            {
                fn abs(&self) -> Self {
                    Self {
                        $x: self.$x.abs(),
                        $y: self.$y.abs(),
                    }
                }
            }

            impl<Unit> Neg for $type<Unit>
            where
                Unit: Neg<Output = Unit>,
            {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    self.map(Unit::neg)
                }
            }

            impl<Unit> IntoUnsigned for $type<Unit>
            where
                Unit: IntoUnsigned,
            {
                type Unsigned = $type<Unit::Unsigned>;

                fn into_unsigned(self) -> Self::Unsigned {
                    self.map(Unit::into_unsigned)
                }
            }

            impl<Unit> IntoSigned for $type<Unit>
            where
                Unit: IntoSigned,
            {
                type Signed = $type<Unit::Signed>;

                fn into_signed(self) -> Self::Signed {
                    self.map(Unit::into_signed)
                }
            }

            impl<Unit> Round for $type<Unit>
            where
                Unit: Round,
            {
                fn round(self) -> Self {
                    self.map(Unit::round)
                }

                fn ceil(self) -> Self {
                    self.map(Unit::ceil)
                }

                fn floor(self) -> Self {
                    self.map(Unit::floor)
                }
            }

            impl<Unit> ScreenScale for $type<Unit>
            where
                Unit: crate::ScreenScale<Lp = Lp, Px = Px, UPx = UPx>,
            {
                type Lp = $type<Lp>;
                type Px = $type<Px>;
                type UPx = $type<UPx>;

                fn into_px(self, scale: crate::Fraction) -> Self::Px {
                    $type {
                        $x: self.$x.into_px(scale),
                        $y: self.$y.into_px(scale),
                    }
                }

                fn from_px(px: Self::Px, scale: crate::Fraction) -> Self {
                    Self {
                        $x: Unit::from_px(px.$x, scale),
                        $y: Unit::from_px(px.$y, scale),
                    }
                }

                fn into_lp(self, scale: crate::Fraction) -> Self::Lp {
                    $type {
                        $x: self.$x.into_lp(scale),
                        $y: self.$y.into_lp(scale),
                    }
                }

                fn from_lp(lp: Self::Lp, scale: crate::Fraction) -> Self {
                    Self {
                        $x: Unit::from_lp(lp.$x, scale),
                        $y: Unit::from_lp(lp.$y, scale),
                    }
                }

                fn into_upx(self, scale: crate::Fraction) -> Self::UPx {
                    $type {
                        $x: self.$x.into_upx(scale),
                        $y: self.$y.into_upx(scale),
                    }
                }

                fn from_upx(px: Self::UPx, scale: crate::Fraction) -> Self {
                    Self {
                        $x: Unit::from_upx(px.$x, scale),
                        $y: Unit::from_upx(px.$y, scale),
                    }
                }
            }

            impl<T> FloatConversion for $type<T>
            where
                T: FloatConversion,
            {
                type Float = $type<T::Float>;

                fn into_float(self) -> Self::Float {
                    $type {
                        $x: self.$x.into_float(),
                        $y: self.$y.into_float(),
                    }
                }

                fn from_float(float: Self::Float) -> Self {
                    $type {
                        $x: T::from_float(float.$x),
                        $y: T::from_float(float.$y),
                    }
                }
            }

            impl<Unit> IntoComponents<Unit> for $type<Unit> {
                fn into_components(self) -> (Unit, Unit) {
                    (self.$x, self.$y)
                }
            }

            impl<Unit> FromComponents<Unit> for $type<Unit> {
                fn from_components(components: (Unit, Unit)) -> Self {
                    Self {
                        $x: components.0,
                        $y: components.1,
                    }
                }
            }

            impl<Unit> Ranged for $type<Unit>
            where
                Unit: Ranged,
            {
                const MAX: Self = Self {
                    $x: Unit::MAX,
                    $y: Unit::MAX,
                };
                const MIN: Self = Self {
                    $x: Unit::MIN,
                    $y: Unit::MIN,
                };
            }

            impl_2d_math!(binary, Add, add, $type, $x, $y);
            impl_2d_math!(assign, AddAssign, add_assign, $type, $x, $y);
            impl_2d_math!(binary, Sub, sub, $type, $x, $y);
            impl_2d_math!(assign, SubAssign, sub_assign, $type, $x, $y);
            impl_2d_math!(binary, Mul, mul, $type, $x, $y);
            impl_2d_math!(assign, MulAssign, mul_assign, $type, $x, $y);
            impl_2d_math!(binary, Div, div, $type, $x, $y);
            impl_2d_math!(assign, DivAssign, div_assign, $type, $x, $y);
            impl_2d_math!(binary, Rem, rem, $type, $x, $y);
            impl_2d_math!(assign, RemAssign, rem_assign, $type, $x, $y);
        }
    };

    (binary $unit:ident, $trait:ident, $method:ident, $type:ident, $x:ident, $y:ident) => {
        impl<Unit> $trait<$unit> for $type<Unit>
        where
            Unit: $trait<$unit, Output = Unit>,
        {
            type Output = Self;

            fn $method(self, rhs: $unit) -> Self::Output {
                Self {
                    $x: self.$x.$method(rhs),
                    $y: self.$y.$method(rhs),
                }
            }
        }
    };
    (binary, $trait:ident, $method:ident, $type:ident, $x:ident, $y:ident) => {
        use std::ops::$trait;

        impl_2d_math!(binary i32, $trait, $method, $type, $x, $y);
        impl_2d_math!(binary f32, $trait, $method, $type, $x, $y);
        impl_2d_math!(binary u32, $trait, $method, $type, $x, $y);
        impl_2d_math!(binary UPx, $trait, $method, $type, $x, $y);
        impl_2d_math!(binary Px, $trait, $method, $type, $x, $y);
        impl_2d_math!(binary Lp, $trait, $method, $type, $x, $y);

        impl<T, Unit> $trait<crate::Point<T>> for $type<Unit>
        where
            Unit: $trait<T, Output = Unit>,
        {
            type Output = Self;

            fn $method(self, rhs: crate::Point<T>) -> Self::Output {
                Self {
                    $x: self.$x.$method(rhs.x),
                    $y: self.$y.$method(rhs.y),
                }
            }
        }

        impl<T, Unit> $trait<crate::Size<T>> for $type<Unit>
        where
            Unit: $trait<T, Output = Unit>,
        {
            type Output = Self;

            fn $method(self, rhs: crate::Size<T>) -> Self::Output {
                Self {
                    $x: self.$x.$method(rhs.width),
                    $y: self.$y.$method(rhs.height),
                }
            }
        }
    };

    (assign, $trait:ident, $method:ident, $type:ident, $x:ident, $y:ident) => {
        use std::ops::$trait;

        impl<Unit> $trait<Unit> for $type<Unit>
        where
            Unit: $trait + Clone,
        {
            fn $method(&mut self, rhs: Unit) {
                self.$x.$method(rhs.clone());
                self.$y.$method(rhs);
            }
        }

        impl<Unit> $trait<crate::Point<Unit>> for $type<Unit>
        where
            Unit: $trait,
        {
            fn $method(&mut self, rhs: crate::Point<Unit>) {
                self.$x.$method(rhs.x);
                self.$y.$method(rhs.y);
            }
        }

        impl<Unit> $trait<crate::Size<Unit>> for $type<Unit>
        where
            Unit: $trait,
        {
            fn $method(&mut self, rhs: crate::Size<Unit>) {
                self.$x.$method(rhs.width);
                self.$y.$method(rhs.height);
            }
        }
    };
}
