use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

use crate::{
    context::Context,
    gl,
    ship::Ship,
};

#[derive(Builder)]
pub struct ShipRendererDescriptor
{
    wing_angle: f32,
    tail_x: f32,
}

/// Renderer for ships.
pub struct ShipRenderer
{
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl ShipRenderer
{
    /// Creates a new `ShipRenderer`.
    pub fn new(
        context: &WebGlRenderingContext,
        descriptor: &ShipRendererDescriptor,
    ) -> Result<ShipRenderer, String>
    {
        //
        // Create program.
        //
        let program = gl::link_program(
            context,
            &vertex_shader(context)?,
            &fragment_shader(context)?,
        )?;

        //
        // Initialize vertex buffer.
        //
        let r = descriptor.wing_angle;
        let vertex_buffer = gl::make_static_draw_array_buffer_f32(
            context,
            vec![
                // 0
                1.0,
                0.0,
                0.0,
                // 1
                r.cos(),
                r.sin(),
                0.0,
                // 2
                descriptor.tail_x,
                0.0,
                0.0,
                // 3
                descriptor.tail_x,
                0.0,
                0.0,
                // 4
                r.cos(),
                -r.sin(),
                0.0,
                // 5
                1.0,
                0.0,
                0.0,
            ],
        )?;

        Ok(ShipRenderer {
            program,
            vertex_buffer,
        })
    }

    /// Renders `ship`.
    pub fn render(&self, context: &Context, ship: &Ship)
    {
        let gl = context.render_context();

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        gl.enable_vertex_attrib_array(0);

        gl.use_program(Some(&self.program));
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        //
        // Calculate world matrix and set the uniform.
        //
        let s = ship.size();
        let p = ship.position();
        let r = ship.yaw();

        let matrix = arr2(&[
            [r.cos(), r.sin(), 0., 0.],
            [-r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
        .dot(&arr2(&[
            [s[0], 0.0, 0.0, 0.0],
            [0.0, s[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
        .dot(&arr2(&[
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [p[0], p[1], 0., 1.],
        ]));

        let location = gl.get_uniform_location(&self.program, "world_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        //
        // Set the perspective matrix uniform
        //
        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        let matrix = arr2(context.foreground_perspective_matrix());
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        //
        // Draw ship.
        //
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        gl.use_program(None);
    }
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_vertex_shader(
        context,
        r#"
    attribute vec4 position;

    uniform mat4 world_matrix;
    uniform mat4 perspective_matrix;

    void main() {
      gl_Position = perspective_matrix * world_matrix * position;
    }
    "#,
    )
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_fragment_shader(
        context,
        r#"
    precision mediump float;

    void main() {
        gl_FragColor = vec4(0.5, 0.8, 0.9, 1.0);
    }
    "#,
    )
}
