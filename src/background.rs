use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::gl;

pub struct Background
{
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
    
        void main() {
            gl_Position = position;
            vertexPosition = position;
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

            float d = sqrt(sq.x + sq.y + sq.z);
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
        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        context.use_program(None);
    }
}
