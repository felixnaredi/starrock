use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::{
    context::Context,
    gl,
};

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

            uniform mat4 model_matrix;
            
            varying vec4 vertex_position;            
    
            void main() {
                vertex_position = model_matrix * position;
                gl_Position = position;
            }
            "#,
        )?;

        let fragment_shader = gl::compile_fragment_shader(
            &context,
            r#"
            #define PI 3.14159265359

            precision mediump float;
            
            uniform mat4 perspective_matrix;

            varying vec4 vertex_position;

            void main() {
                vec4 v0 = vec4(vertex_position.y, vertex_position.x, 1.0, 1.0);
                vec4 v1 = perspective_matrix * v0;

                vec4 d0 = v1 * v1;
                float d1 = sqrt(d0.x + d0.y + d0.z);
                float d2 = d1 * d1;

                gl_FragColor = vec4(sin(d2 * 17.0), 0.2, cos(d2 * 29.0), 1.0);
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

    pub fn render(&self, context: &Context)
    {
        let render_context = context.render_context().unwrap();

        render_context.use_program(Some(&self.program));

        render_context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        render_context.enable_vertex_attrib_array(0);

        let model_matrix_location =
            render_context.get_uniform_location(&self.program, "model_matrix");
        let transpose = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0],
        ]);

        render_context.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            transpose.view().as_slice().unwrap(),
        );

        let location = render_context.get_uniform_location(&self.program, "perspective_matrix");
        let matrix = arr2(&context.perspective_matrix().unwrap_or([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]));
        render_context.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        render_context.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        render_context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        render_context.use_program(None);
    }
}
