use getset::{
    Getters,
    Setters,
};
use web_sys::WebGlRenderingContext;

#[derive(Getters, Setters)]
pub struct Context
{
    #[getset(set = "pub")]
    render_context: Option<WebGlRenderingContext>,

    #[getset(get = "pub", set = "pub")]
    perspective_matrix: Option<[[f32; 4]; 4]>,
}

impl Context
{
    pub fn new() -> Context
    {
        Context {
            render_context: None,
            perspective_matrix: None,
        }
    }

    pub fn render_context(&self) -> Option<&WebGlRenderingContext>
    {
        self.render_context.as_ref()
    }

    pub fn with_render_context(mut self, render_context: WebGlRenderingContext) -> Context
    {
        self.render_context = Some(render_context);
        self
    }

    pub fn with_perspective_matrix(mut self, perspective_matrix: [[f32; 4]; 4]) -> Context
    {
        self.perspective_matrix = Some(perspective_matrix);
        self
    }
}
