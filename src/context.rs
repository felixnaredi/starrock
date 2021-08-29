use getset::{
    Getters,
    Setters,
};
use web_sys::WebGlRenderingContext;

#[derive(Builder)]
pub struct ContextDescriptor
{
    canvas_width: u32,
    canvas_height: u32,
    render_context: WebGlRenderingContext,
    foreground_perspective_matrix: [[f32; 4]; 4],
}

#[derive(Getters, Setters)]
pub struct Context
{
    #[getset(get = "pub")]
    canvas_width: u32,

    #[getset(get = "pub")]
    canvas_height: u32,

    #[getset(get = "pub")]
    render_context: WebGlRenderingContext,

    #[getset(get = "pub")]
    foreground_perspective_matrix: [[f32; 4]; 4],
    // foreground_texture: WebGlTexture,
    // foreground_texture_framebuffer: WebGlFramebuffer,
}

impl Context
{
    pub fn new(descriptor: ContextDescriptor) -> Context
    {
        Context {
            canvas_width: descriptor.canvas_width,
            canvas_height: descriptor.canvas_height,
            render_context: descriptor.render_context,
            foreground_perspective_matrix: descriptor.foreground_perspective_matrix,
        }
    }
}
