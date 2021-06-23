use std::f32::consts::PI;

use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::gl;

pub struct Ship
{
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl Ship
{
    pub fn new(context: &WebGlRenderingContext) -> Result<Ship, String>
    {
        let vertex_shader = gl::compile_vertex_shader(
            &context,
            r#"
            attribute vec4 position;

            uniform mat4 model_matrix;
    
            void main() {
              gl_Position = model_matrix * position;
            }
            "#,
        )?;

        let fragment_shader = gl::compile_fragment_shader(
            &context,
            r#"
            void main() {
                gl_FragColor = vec4(0.5, 0.2, 0.3, 1.0);
            }
            "#,
        )?;

        let program = gl::link_program(&context, &vertex_shader, &fragment_shader)?;

        let r = PI * (22.0 / 36.0);
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
            -1.0 / 6.0,
            0.0,
            0.0,
            // 3
            -1.0 / 6.0,
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

        Ok(Ship {
            program,
            vertex_buffer,
        })
    }

    pub fn draw(&self, context: &WebGlRenderingContext)
    {
        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        context.enable_vertex_attrib_array(0);

        context.use_program(Some(&self.program));
        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        let model_matrix_location = context.get_uniform_location(&self.program, "model_matrix");
        let scale = arr2(&[
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let transpose = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-0.33, -0.25, 0.0, 1.0],
        ]);

        context.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            (scale.dot(&transpose)).view().as_slice().unwrap(),
        );

        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        context.use_program(None);
    }
}
