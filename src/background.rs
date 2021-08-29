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

            uniform mat4 world_matrix;
            uniform mat4 perspective_matrix;
            
            varying vec4 vertex_position;            
    
            void main() {
                vertex_position = perspective_matrix * world_matrix * position;
                gl_Position = position;
            }
            "#,
        )?;

        let fragment_shader = gl::compile_fragment_shader(
            &context,
            r#"
            #define PI 3.14159265359

            precision mediump float;        

            varying vec4 vertex_position;

            void main() {
                vec4 d0 = vertex_position * vertex_position;
                float d1 = sqrt(d0.x + d0.y + d0.z);
                float d2 = d1 * d1;
                
                gl_FragColor = vec4(abs(sin(d2 * 17.0)) * 0.3, 
                                    0.1, 
                                    abs(cos(d2 * 29.0)) * 0.3,
                                    1.0);
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
        let gl = context.render_context();

        gl.use_program(Some(&self.program));

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        gl.enable_vertex_attrib_array(0);

        let location = gl.get_uniform_location(&self.program, "world_matrix");
        let matrix = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0],
        ]);
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        let w = context.canvas_width().clone() as f32;
        let h = context.canvas_height().clone() as f32;
        let matrix = arr2(&[
            [1., 0., 0., 0.],
            [0., h / w, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        gl.use_program(None);
    }
}
