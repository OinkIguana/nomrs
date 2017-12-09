//! Assorted utility functions

/// Splits a floating point number `f` into a pair of integers `(i, e)`, such that `f = i * 2^e`.
///
/// Passing `±Infinity` or `NaN` will return produce `(0, 0)`, as they should not be passed to this
/// function and seem to be not supported by Noms.
pub fn frexp(f: f64) -> (i64, i64) {
    if !f.is_normal() {
        return (0i64, 0i64)
    }
    let mut e = 0f64;
    while (f / e.exp2()).floor() != (f / e.exp2()) {
        e -= 1f64;
    }
    let mut b = (f / e.exp2()) as i64;
    while b % 2i64 == 0 {
        b = b / 2i64;
        e += 1f64;
    }
    ((f / e.exp2()) as i64, e as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frexp_zero() {
        assert_eq!( frexp(0f64), (0, 0) );
        assert_eq!( frexp(::std::f64::INFINITY), (0, 0) );
        assert_eq!( frexp(-::std::f64::INFINITY), (0, 0) );
    }
    #[test]
    fn frexp_int() {
        assert_eq!( frexp(1f64), (1, 0) );
        assert_eq!( frexp(5f64), (5, 0) );
    }
    #[test]
    fn frexp_reduce() {
        assert_eq!( frexp(2f64), (1, 1) );
        assert_eq!( frexp(4f64), (1, 2) );
        assert_eq!( frexp(6f64), (3, 1) );
    }
    #[test]
    fn frexp_frac() {
        assert_eq!( frexp(2.5), (5, -1) );
        assert_eq!( frexp(0.5), (1, -1) );
        assert_eq!( frexp(0.25), (1, -2) );
        assert_eq!( frexp(124.25), (497, -2) )
    }
}
