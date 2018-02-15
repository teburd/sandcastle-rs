/// Render node which will draw geometry with a given material (color, texture, gradient, shader)
pub struct Geometry<N> {
    geometry: Vec<N>,
    material: Box<Material>,
}
