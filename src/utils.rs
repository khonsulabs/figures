use std::cmp::Ordering;
use std::ops::Mul;

/// Performs `value as i32`.
///
/// This function exists solely because of clippy. In some situations, the only
/// way to convert from f32 to i32 is the `as` keyword, because truncating
/// floating point values is desired.
#[allow(clippy::cast_possible_truncation)] // truncation desired
#[must_use]
pub fn lossy_f32_to_i32(value: f32) -> i32 {
    value as i32
}

/// Performs `value as f32`.
///
/// This function exists solely because of clippy. The truncation of f64 -> f32
/// isn't as severe as truncation of integer types, but it's lumped into the
/// same lint. I don't want to disable the truncation lint, and I don't want
/// functions that need to do this operation to not be checking for integer
/// truncation.
#[allow(clippy::cast_possible_truncation)] // truncation desired
#[must_use]
pub fn lossy_f64_to_f32(value: f64) -> f32 {
    value as f32
}

/// Orders two vectors by their magnitude, then by their individual component
/// values. E.g., this list is ordered consistently with the results of this
/// function:
///
/// - `(1, 1)`
/// - `(2, 2)`
/// - `(2, 4)`
/// - `(3, 3)`
/// - `(2, 6)`
/// - `(3, 4)`
/// - `(2, 8)`
/// - `(4, 4)`
///
/// This ordering ensures that for any values where `this == other`,
/// `Ordering::Equal` is returned, while any values where `this != other`,
/// either `Ordering::Less` or `Ordering::Greater` are returned.
pub(crate) fn vec_ord<Unit>(this: (Unit, Unit), other: (Unit, Unit)) -> Ordering
where
    Unit: Ord + Copy + Mul<Output = Unit>,
{
    // Goal: Sort so that vectors are ordered by their magnitude. This isn't
    // good enough, however, as Ordering::Equal will be returned for items that
    // Eq does not return true for. To ensure that Ordering::Equal is only
    // returned for vecs that Eq returns true for, we further sort by the
    // smallest component.
    let this_magnitude = this.0 * this.1;
    let other_magnitude = other.0 * other.1;
    match this_magnitude.cmp(&other_magnitude) {
        Ordering::Equal => {
            match (this.0.cmp(&other.0), this.1.cmp(&other.1)) {
                (Ordering::Less | Ordering::Equal, Ordering::Less)
                | (Ordering::Less, Ordering::Equal) => Ordering::Less,
                (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
                (Ordering::Equal | Ordering::Greater, Ordering::Greater)
                | (Ordering::Greater, Ordering::Equal) => Ordering::Greater,

                // Width and height vary. Sort based on the smallest measurement.
                // Since we already know one set of comparison results, we only need
                // to do one or two more comparisons to determine which measurement
                // is the smallest.
                (Ordering::Less, Ordering::Greater) => {
                    compare_smallest(this.1, this.0, other.1, other.0)
                }
                (Ordering::Greater, Ordering::Less) => {
                    compare_smallest(this.0, this.1, other.0, other.1)
                }
            }
        }
        other => other,
    }
}

fn compare_smallest<Unit>(a1: Unit, a2: Unit, b1: Unit, b2: Unit) -> Ordering
where
    Unit: Ord + Copy,
{
    match b1.cmp(&a2) {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => {
            if b2 < a1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Ordering::Greater => Ordering::Less,
    }
}
