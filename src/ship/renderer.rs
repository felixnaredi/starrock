use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

use crate::{
    context::Context,
    gl,
    matrix::{
        Matrix4x4,
        Rotate,
        Scale,
        Translate,
    },
    ship::Ship,
};

/// Renderer for ships.
pub struct ShipRenderer
{
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl ShipRenderer
{
    /// Creates a new `ShipRenderer`.
    pub fn new(context: &WebGlRenderingContext, ship: &Ship) -> Result<ShipRenderer, String>
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
        let r = ship.wing_angle();
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
                *ship.tail_x(),
                0.0,
                0.0,
                // 3
                *ship.tail_x(),
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
        let matrix = Scale::id()
            .vec2(ship.size())
            .into_arr2()
            .dot(&Rotate::id().radians(*ship.yaw()).into_arr2())
            .dot(&Translate::id().vec2(ship.position()).into_arr2());

        let location = gl.get_uniform_location(&self.program, "world_matrix");

        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        //
        // Set the projection matrix uniform
        //
        let matrix = context.foreground_projection_matrix().clone().into_array();
        let location = gl.get_uniform_location(&self.program, "projection_matrix");

        gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &matrix);

        //
        // Draw ship.
        //
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        //
        // Clean-up
        //
        gl.use_program(None);
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_vertex_shader(
        context,
        r#"
        attribute vec4 position;

        uniform mat4 world_matrix;
        uniform mat4 projection_matrix;

        void main()
        {
            gl_Position = projection_matrix * world_matrix * position;
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

        void main()
        {
            gl_FragColor = vec4(0.5, 0.8, 0.9, 1.0);
        }
    "#,
    )
}
