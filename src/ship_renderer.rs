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
    pub fn new(
        context: &WebGlRenderingContext,
        descriptor: &ShipRendererDescriptor,
    ) -> Result<ShipRenderer, String>
    {
        let program = gl::link_program(
            context,
            &vertex_shader(context)?,
            &fragment_shader(context)?,
        )?;

        let r = descriptor.wing_angle;
        let vertices: [f32; 18] = [
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
        ];

        let vertex_buffer = context.create_buffer().ok_or("failed to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        Ok(ShipRenderer {
            program,
            vertex_buffer,
        })
    }

    pub fn render(&self, context: &Context, ship: &Ship)
    {
        let gl = context.render_context().unwrap();

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        gl.enable_vertex_attrib_array(0);

        gl.use_program(Some(&self.program));
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        let model_matrix_location = gl.get_uniform_location(&self.program, "model_matrix");

        let s = ship.size();
        let scale = arr2(&[
            [s[0], 0.0, 0.0, 0.0],
            [0.0, s[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = ship.position();
        let transpose = arr2(&[
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [p[0], p[1], 0., 1.],
        ]);

        let r = ship.yaw();
        let yaw = arr2(&[
            [r.cos(), r.sin(), 0., 0.],
            [-r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        gl.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            (yaw.dot(&scale).dot(&transpose)).view().as_slice().unwrap(),
        );

        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        let matrix = arr2(&context.perspective_matrix().unwrap_or([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]));
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

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

    uniform mat4 model_matrix;
    uniform mat4 perspective_matrix;

    void main() {
      gl_Position = perspective_matrix * model_matrix * position;
    }
    "#,
    )
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_fragment_shader(
        context,
        r#"
    void main() {
        gl_FragColor = vec4(0.5, 0.7, 0.8, 1.0);
    }
    "#,
    )
}
