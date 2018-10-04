use std::f64::consts::PI;

/// Calculate the difference between two circular radian angles.
///
/// The returned value will always be `(-PI, PI]`.
#[inline(always)]
pub fn diff(a: f64, b: f64) -> f64 {
    bound(a - b)
}

/// Map the given angle value within the radian range `(-PI, PI]`.
#[inline(always)]
pub fn bound(a: f64) -> f64 {
    (a + PI).mod_euc(2.0 * PI) - PI
}
