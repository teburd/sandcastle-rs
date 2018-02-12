/// RenderNode trait all render nodes must implement
pub trait RenderNode {
    fn parent(&self) -> Option<Box<RenderNode>>;
    fn children(&self) -> Iterator<Item=Box<RenderNode>>;
}
