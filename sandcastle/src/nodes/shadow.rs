use euclid::Rect;
use image::Rgba;

/// Simple outset shadow around a given bounds rectangle
pub struct Shadow {
    bounds: Rect<f64>,
    dx: f64,
    dy: f64,
    color: Rgba<u8>,
}
