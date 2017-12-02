//! Assorted utility functions

/// Splits a floating point number `f` into a pair of integers `(i, e)`, such that `f = i * 2^e`.
///
/// Passing `±Infinity` or `NaN` will return produce `(0, 0)`, as they should not be passed to this
/// function and seem to be not supported by Noms.
///
/// TODO: reduce the `i` and increase `e` while possible, to match the bytes from the official Noms
///       version of this function exactly
pub fn frexp(f: f64) -> (i64, i64) {
    if !f.is_normal() {
        return (0i64, 0i64)
    }
    let mut e = 0f64;
    while (f / e.exp2()).floor() != (f / e.exp2()) {
        e -= 1f64;
    }
    ((f / e.exp2()) as i64, e as i64)
}
