pub fn lerp(a: f64, b: f64, interpolation: f64) -> f64 {
    a + interpolation * (b - a)
}
