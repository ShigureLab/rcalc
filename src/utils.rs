#[allow(dead_code)]
pub fn assert_close(a: f64, b: f64) {
    assert!(f64::abs(a - b) < 1e-6);
}
