use RenderNode;
use euclid::Transform3D;

/// Transform a given render node by applying a built transformation matrix
#[derive(Debug)]
pub struct Transform<N: RenderNode> {
    child: N, 
    transform: Transform3D<f64>
}


impl<N: RenderNode> Transform<N> {
    pub fn new(child: N, transform: Transform3D<f64>) -> Transform<N> {
        Transform {
            child: child,
            transform: transform,
        }
    }

    pub fn child(&self) -> &RenderNode {
        &self.child
    }

    pub fn transform(&self) -> &Transform3D<f64> {
        &self.transform
    }
}
