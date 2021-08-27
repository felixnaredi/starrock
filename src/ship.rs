use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::gl;

#[derive(Builder)]
pub struct ShipDescriptor
{
    size: [f32; 2],
    position: [f32; 2],
    yaw: f32,
    wing_angle: f32,
    tail_x: f32,
}

pub struct Ship
{
    position: [f32; 2],
    size: [f32; 2],
    yaw: f32,
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl Ship
{
    pub fn new(context: &WebGlRenderingContext, descriptor: &ShipDescriptor)
        -> Result<Ship, String>
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
                gl_FragColor = vec4(0.5, 0.7, 0.8, 1.0);
            }
            "#,
        )?;

        let program = gl::link_program(&context, &vertex_shader, &fragment_shader)?;

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

        Ok(Ship {
            position: descriptor.position,
            size: descriptor.size,
            yaw: descriptor.yaw,
            program,
            vertex_buffer,
        })
    }

    pub fn increase_yaw(&mut self, amount: f32)
    {
        self.yaw += amount;
    }

    pub fn move_forward(&mut self, amount: f32)
    {
        self.position[0] += amount * self.yaw.cos();
        self.position[1] += amount * self.yaw.sin();
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

        let s = self.size;
        let scale = arr2(&[
            [s[0], 0.0, 0.0, 0.0],
            [0.0, s[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = self.position;
        let transpose = arr2(&[
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [p[0], p[1], 0., 1.],
        ]);

        let r = self.yaw;
        let yaw = arr2(&[
            [r.cos(), r.sin(), 0., 0.],
            [-r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        context.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            (yaw.dot(&scale).dot(&transpose)).view().as_slice().unwrap(),
        );

        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        context.use_program(None);
    }
}
