use std::{
    f32::consts::PI,
    iter,
};

use ndarray::arr2;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
};

use crate::gl;

#[derive(Builder)]
pub struct RockDescriptor
{
    sides: i32,
    size: [f32; 2],
    position: [f32; 2],
}

pub struct Rock
{
    sides: i32,
    size: [f32; 2],
    position: [f32; 2],
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

fn thrice<T>(x: T, y: T, z: T) -> impl Iterator<Item = T>
{
    iter::once(x).chain(iter::once(y)).chain(iter::once(z))
}

impl Rock
{
    pub fn new(context: &WebGlRenderingContext, descriptor: &RockDescriptor)
        -> Result<Rock, String>
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
                gl_FragColor = vec4(0.10, 0.10, 0.05, 1.00);
            }
            "#,
        )?;

        let program = gl::link_program(&context, &vertex_shader, &fragment_shader)?;

        let sides = descriptor.sides;
        let r = 2.0 * PI * (1.0 / (sides as f32));

        let vertices: Vec<f32> = iter::once(thrice(0.0, 0.0, 0.0))
            .chain(
                (0..sides)
                    .map(|i| i as f32 * r)
                    .map(|r| thrice(r.cos(), r.sin(), 0.0)),
            )
            .chain(iter::once(thrice(1.0, 0.0, 0.0)))
            .flatten()
            .collect();

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

        Ok(Rock {
            sides,
            size: descriptor.size,
            position: descriptor.position,
            program,
            vertex_buffer,
        })
    }

    pub fn update(&mut self)
    {
        let x = &mut self.position[0];
        *x += 0.01 * (1. - self.size[0]);
        if *x > 1.0 {
            *x = -1.0;
        }
        if *x < -1.0 {
            *x = 1.0;
        }

        let dy = if self.sides % 2 == 0 { 1. } else { -1. };
        let y = &mut self.position[1];
        *y += dy * 0.01 * (1. - self.size[1] * 3.);
        if *y > 1.0 {
            *y = -1.0;
        }
        if *y < -1.0 {
            *y = 1.0;
        }
    }

    pub fn draw(&self, context: &WebGlRenderingContext)
    {
        context.use_program(Some(&self.program));

        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );

        context.enable_vertex_attrib_array(0);
        context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        let model_matrix_location = context.get_uniform_location(&self.program, "model_matrix");
        let scale = arr2(&[
            [self.size[0], 0.0, 0.0, 0.0],
            [0.0, self.size[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let transpose = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0],
        ]);

        context.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            (scale.dot(&transpose)).view().as_slice().unwrap(),
        );

        context.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, self.sides + 2);

        context.use_program(None);
    }
}
