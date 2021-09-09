use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

use crate::{
    bullet::Bullet,
    context::Context,
    gl,
    matrix::{
        Matrix4x4,
        Rotate,
        Scale,
        Translate,
    },
};

pub struct BulletRenderer
{
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl BulletRenderer
{
    pub fn new(context: &Context) -> Result<BulletRenderer, String>
    {
        let gl = context.render_context();

        let program = gl::link_program(gl, &vertex_shader(gl)?, &fragment_shader(gl)?)?;

        let vertex_buffer = gl::make_static_draw_array_buffer_f32(
            gl,
            vec![-1., -1., 0., -1., 1., 0., 1., -1., 0., 1., 1., 0.],
        )?;

        Ok(BulletRenderer {
            program,
            vertex_buffer,
        })
    }

    pub fn render(&self, context: &Context, bullet: &Bullet)
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
            .vec2(bullet.size())
            .into_arr2()
            .dot(&Rotate::id().vec2(*bullet.velocity()).into_arr2())
            .dot(&Translate::id().vec2(bullet.position()).into_arr2());

        let location = gl.get_uniform_location(&self.program, "world_matrix");

        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        //
        // Set the projection matrix uniform.
        //
        let matrix = context.foreground_projection_matrix().clone().into_array();
        let location = gl.get_uniform_location(&self.program, "projection_matrix");

        gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &matrix);

        //
        // Draw ship.
        //
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

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
        &context,
        r#"
        attribute vec4 position;

        uniform mat4 world_matrix;
        uniform mat4 projection_matrix;

        varying vec4 vertex_position;

        void main()
        {
            vec4 p0 = projection_matrix * world_matrix * position;

            vertex_position = p0;
            gl_Position = p0;
        }
        "#,
    )
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_fragment_shader(
        &context,
        r#"
        #define PI 3.14159265359

        precision mediump float;

        varying vec4 vertex_position;

        void main()
        {
            vec4 d0 = vertex_position * vertex_position;
            float d1 = sqrt(d0.x + d0.y + d0.z);
            float d2 = d1 * d1;

            gl_FragColor = vec4(abs(sin(d2 * 17.0)) * 0.8,
                                0.8,
                                abs(cos(d2 * 29.0)) * 0.8,
                                1.0);
        }
        "#,
    )
}
