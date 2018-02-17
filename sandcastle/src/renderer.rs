use error::RenderResult;
use render_node::RenderNode;

/// Renderer only requires one method be implemented
pub trait Renderer {
    fn render(&mut self, node: Box<RenderNode>) -> RenderResult<()>;
}
