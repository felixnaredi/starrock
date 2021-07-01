use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::gl;

pub struct Background
{
    pub position: [f32; 2],
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl Background
{
    pub fn new(context: &WebGlRenderingContext) -> Result<Background, String>
    {
        let vertex_shader = gl::compile_vertex_shader(
            &context,
            r#"
            attribute vec4 position;
            varying vec4 vertexPosition;

            uniform mat4 model_matrix;
    
            void main() {
                gl_Position = position;
                vertexPosition = model_matrix * position;
            }
            "#,
        )?;

        let fragment_shader = gl::compile_fragment_shader(
            &context,
            r#"
            #define PI 3.14159265359

            precision mediump float;
            varying vec4 vertexPosition;

            void main() {
                vec4 sq = vertexPosition * vertexPosition;

                float d0 = sqrt(sq.x + sq.y + sq.z);
                float d = d0 * d0;
                float r = (sin(d * PI * 4.8) + 1.0) / 2.0;
                float b = (sin(d * PI * 1.8) + 1.0) / 2.0;

                gl_FragColor = vec4(r, 0.3, b, 1.0);
            }
            "#,
        )?;

        let program = gl::link_program(context, &vertex_shader, &fragment_shader)?;

        let vertices: [f32; 18] = [
            1.0, 1.0, 0.0, // 0
            -1.0, 1.0, 0.0, // 1
            -1.0, -1.0, 0.0, // 2
            -1.0, -1.0, 0.0, // 3
            1.0, -1.0, 0.0, // 4
            1.0, 1.0, 0.0, // 5
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

        Ok(Background {
            position: [0.0, 0.0],
            program,
            vertex_buffer,
        })
    }

    pub fn draw(&self, context: &WebGlRenderingContext)
    {
        context.use_program(Some(&self.program));

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        context.enable_vertex_attrib_array(0);

        let model_matrix_location = context.get_uniform_location(&self.program, "model_matrix");
        let transpose = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0],
        ]);

        context.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            transpose.view().as_slice().unwrap(),
        );

        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        context.use_program(None);
    }
}
