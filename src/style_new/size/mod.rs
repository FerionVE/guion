#[non_exhaustive]
pub enum StdUnit { //TODO use float with smart eq in cachors (e.g. int(f+(1/64)*(1/32)))
    /// Pixel unit
    Px(i32),
    /// DPI-independent destination pixel unit
    RawPx(i32),
    /// Relative to current font size
    Em(i32),
}

// Floats only eq'd in destination unit after appling scale/dpi
fn rough_eq_32(a: f32, b: f32) -> bool {
    (a*64.).round() == (b*64.).round()
}
fn rough_eq_64(a: f64, b: f64) -> bool {
    (a*64.).round() == (b*64.).round()
}
