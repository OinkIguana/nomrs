pub fn frexp(f: f64) -> (i64, i64) {
    let mut e = 0f64;
    while (f / e.exp2()).floor() != (f / e.exp2()) {
        e -= 1f64;
    }
    ((f / e.exp2()) as i64, e as i64)
}
