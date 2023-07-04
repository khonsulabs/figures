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
