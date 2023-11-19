use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::traits::{FromComponents, IntoComponents, ScreenScale};
use crate::units::{Lp, Px, UPx};
use crate::{Fraction, Point, Size};

#[test]
fn one_inch_is_correct() {
    assert_eq!(Lp::inches(1).into_px(Fraction::ONE), Px::new(96));
    assert_eq!(Px::new(96).into_lp(Fraction::ONE), Lp::inches(1));
    assert_eq!(
        Lp::inches(1).into_px(Fraction::new_whole(2)),
        Px::new(96 * 2)
    );
    assert_eq!(Lp::inches(1).into_upx(Fraction::ONE), UPx::new(96));
    assert_eq!(UPx::new(96).into_lp(Fraction::ONE), Lp::inches(1));
}

#[test]
fn lp_conversions() {
    assert_eq!(Lp::cm_f(1.), Lp::cm(1));
    assert_eq!(Lp::mm_f(1.), Lp::mm(1));
    assert_eq!(Lp::inches_f(1.), Lp::inches(1));
    assert_eq!(Lp::mm(10), Lp::cm(1));
    assert_eq!(Lp::inches(1), Lp::cm_f(2.54));
}

#[test]
fn ratio_simplification() {
    assert_eq!(Fraction::new(2, 3) * Fraction::new(3, 2), Fraction::ONE);
}

#[test]
fn ratio_equality() {
    assert_eq!(Fraction::new(2, 3), Fraction::new(4, 6));
}

#[test]
fn scale_factor_from_f32() {
    let factor = Fraction::from(1.0 / 3.0);
    assert_eq!(factor, Fraction::new(1, 3));
    let factor = Fraction::from(16.0 / 9.0);
    assert_eq!(factor, Fraction::new(16, 9));
    let factor = Fraction::from(3. / 4.);
    assert_eq!(factor, Fraction::new(3, 4));
    let factor = Fraction::from(-3. / 4.);
    assert_eq!(factor, Fraction::new(-3, 4));
}

#[test]
fn inverse_ratio() {
    assert_eq!(Fraction::new(1, 3).inverse(), Fraction::new(3, 1));
    assert_eq!(
        Fraction::new(1, i16::MAX).inverse(),
        Fraction::new(i16::MAX, 1)
    );
    // Negative minimum inverse
    assert_eq!(
        Fraction::new(-32_767, 1).inverse(),
        Fraction::new(-1, 32_767)
    );
}

#[allow(clippy::eq_op)]
fn test_unit_math<Unit, E>()
where
    Unit: TryFrom<i32, Error = E>
        + Mul<Output = Unit>
        + Div<Output = Unit>
        + MulAssign
        + DivAssign
        + Add<Output = Unit>
        + AddAssign
        + Sub<Output = Unit>
        + SubAssign
        + Eq
        + Ord
        + Debug
        + Copy,

    E: Debug,
{
    let num = (0..=10)
        .map(Unit::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    // Test comparisons
    assert_ne!(num[0], num[1]);
    assert_eq!(num[0], num[0]);
    assert!(num[0] < num[1]);
    assert!(num[1] > num[0]);
    assert!(num[1] == num[1]);

    assert_eq!(num[1] + num[1], num[2]);
    assert_eq!(num[2] - num[1], num[1]);
    assert_eq!(num[2] * num[3], num[6]);
    assert_eq!(num[4] / num[2], num[2]);

    let mut x = num[1];
    x += num[2];
    x -= num[1];
    x *= num[3];
    x /= num[2];
    assert_eq!(x, num[3]);

    test_vec_math::<Point<Unit>, _>(&num);
    test_vec_math::<Size<Unit>, _>(&num);
}

fn test_vec_math<Vec, Unit>(num: &[Unit])
where
    Vec: FromComponents<Unit> + Debug + Copy,
    Vec: Add<Output = Vec>
        + Sub<Output = Vec>
        + Mul<Output = Vec>
        + Div<Output = Vec>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Eq,
    Unit: Copy,
{
    macro_rules! v {
        ($x:expr, $y:expr) => {
            Vec::from_components(($x, $y))
        };
    }

    let a = v!(num[2], num[3]);
    let b = v!(num[5], num[3]);
    assert_eq!(a + b, v!(num[7], num[6]));
    assert_eq!(b - a, v!(num[3], num[0]));
    assert_eq!(a * b, v!(num[10], num[9]));
    assert_eq!(b / a, v!(num[2], num[1]));

    let mut x = a;
    x += v![num[3], num[2]];
    x -= v![num[2], num[1]];
    x *= v![num[3], num[2]];
    x /= v![num[3], num[4]];
    assert_eq!(x, v!(num[3], num[2]));
}

#[test]
fn math_ops() {
    test_unit_math::<Px, _>();
    test_unit_math::<UPx, _>();
    test_unit_math::<Lp, _>();
}

fn test_vec_ord<V>()
where
    V: Ord + FromComponents<i32> + IntoComponents<i32> + Debug + Copy,
{
    let mut sizes = Vec::new();
    for width in 1..10 {
        for height in 1..10 {
            sizes.push(V::from_components((width, height)));
        }
    }
    sizes.sort();
    println!("Sorted: {sizes:#?}");
    for sizes in sizes.windows(2) {
        assert!(
            sizes[0] <= sizes[1],
            "{:?} is not less than {:?}",
            sizes[0],
            sizes[1]
        );
        let a = sizes[0].into_components();
        let area1 = a.0 * a.1;
        let b = sizes[1].into_components();
        let area2 = b.0 * b.1;
        assert!(
            area1 <= area2,
            "{:?} sorted less than {:?}, but it's area is larger",
            sizes[0],
            sizes[1]
        );
    }
}

#[test]
fn size_ord() {
    test_vec_ord::<Size<i32>>();
}

#[test]
fn point_ord() {
    test_vec_ord::<Point<i32>>();
}

#[test]
fn size_ord_and_eq() {
    // Sizes of equal area, but different measurements.
    let a = Size {
        width: 2,
        height: 8,
    };
    let b = Size {
        width: 4,
        height: 4,
    };
    assert_ne!(a, b);
    assert!(a < b);
    assert_eq!(a.area(), b.area());
}
