use image::{Pixel, ImageBuffer};

/// A node in the scene graph which contains an Image
#[derive(Debug)]
pub struct Image<P: Pixel, C> {
    buffer: ImageBuffer<P, C>,
}

impl<P: Pixel, C> Image<P, C> {
    fn new(buffer: ImageBuffer<P, C>) -> Image<P, C> {
        Image {
            buffer: buffer
        }
    }

    fn buffer(&self) -> &ImageBuffer<P, C> {
        &self.buffer
    }
}
