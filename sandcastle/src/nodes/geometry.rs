use materials::Material;

/// Render node which will draw geometry with a given material (color, texture, gradient, shader)
pub struct Geometry<N, M: Material> {
    geometry: Vec<N>,
    material: M,
}
