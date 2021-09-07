use getset::{
    Getters,
    Setters,
};
use web_sys::WebGlRenderingContext;

use crate::matrix::OrthographicProjection;

#[derive(Builder)]
pub struct ContextDescriptor
{
    canvas_width: u32,
    canvas_height: u32,
    render_context: WebGlRenderingContext,
    foreground_projection_matrix: OrthographicProjection,
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
    foreground_projection_matrix: OrthographicProjection,
}

impl Context
{
    pub fn new(descriptor: ContextDescriptor) -> Context
    {
        Context {
            canvas_width: descriptor.canvas_width,
            canvas_height: descriptor.canvas_height,
            render_context: descriptor.render_context,
            foreground_projection_matrix: descriptor.foreground_projection_matrix,
        }
    }
}
