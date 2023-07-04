use crate::traits::IntoPixels;
use crate::units::{Dip, Px};
use crate::Ratio;

#[test]
fn one_inch_is_correct() {
    assert_eq!(Dip::INCH.into_pixels(Ratio::ONE), Px(96));
}

#[test]
fn ratio_simplification() {
    assert_eq!(Ratio::new(2, 3) * Ratio::new(3, 2), Ratio::ONE);
}

#[test]
fn ratio_equality() {
    assert_eq!(Ratio::new(2, 3), Ratio::new(4, 6));
}

#[test]
fn scale_factor_from_f32() {
    let factor = Ratio::from_f32(1.0 / 3.0);
    assert_eq!(factor, Ratio::new(1, 3));
    let factor = Ratio::from_f32(16.0 / 9.0);
    assert_eq!(factor, Ratio::new(16, 9));
    let factor = Ratio::from_f32(3. / 4.);
    assert_eq!(factor, Ratio::new(3, 4));
    let factor = Ratio::from_f32(-3. / 4.);
    assert_eq!(factor, Ratio::new(-3, 4));
}

#[test]
fn inverse_ratio() {
    assert_eq!(Ratio::new(1, 3).inverse(), Some(Ratio::new(3, 1)));
    // Overflowing inverse
    assert_eq!(Ratio::new(1, u16::MAX).inverse(), None);
    // Negative minimum inverse
    assert_eq!(
        Ratio::new(-32_767, 1).inverse(),
        Some(Ratio::new(-1, 32_767))
    );
}
