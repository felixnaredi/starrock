use std::{
    collections::HashMap,
    f32::consts::PI,
    iter,
};

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
    rock::Rock,
    rock_shape::RockShape,
};

/// Renderer that renders a `Rock` into a canvas.
pub struct RockRenderer
{
    program: WebGlProgram,
    vertex_buffers: HashMap<RockShape, WebGlBuffer>,
}

impl RockRenderer
{
    pub fn new(context: &WebGlRenderingContext) -> Result<RockRenderer, String>
    {
        let program = gl::link_program(
            context,
            &vertex_shader(context)?,
            &fragment_shader(context)?,
        )?;

        let vertex_buffers: Result<_, String> = RockShape::iter()
            .map(|shape| {
                let buffer = gl::make_static_draw_array_buffer_f32(
                    context,
                    polygon_vertices(shape.sides()).unwrap(),
                )?;
                Ok((shape, buffer))
            })
            .collect();
        let vertex_buffers = vertex_buffers?;

        Ok(RockRenderer {
            program,
            vertex_buffers,
        })
    }

    pub fn render(&self, context: &Context, rock: &Rock)
    {
        let gl = context.render_context();

        gl.use_program(Some(&self.program));

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffers[rock.shape()]),
        );

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        let s = rock.size();
        let scale = arr2(&[
            [s[0], 0.0, 0.0, 0.0],
            [0.0, s[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = rock.position();
        let transpose = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [p[0], p[1], 0.0, 1.0],
        ]);

        let model_matrix_location = gl.get_uniform_location(&self.program, "model_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            model_matrix_location.as_ref(),
            false,
            (scale.dot(&transpose)).view().as_slice().unwrap(),
        );

        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        let matrix = arr2(context.foreground_perspective_matrix());
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLE_FAN,
            0,
            rock.shape().sides() as i32 + 2,
        );

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
                gl_FragColor = vec4(0.10, 0.10, 0.05, 1.00);
            }
            "#,
    )
}

fn polygon_vertices(n: u32) -> Option<Vec<f32>>
{
    (n > 2).then(|| ())?;

    let r = 2.0 * PI * (1.0 / (n as f32));
    Some(
        iter::once(xyz(0.0, 0.0, 0.0))
            .chain(
                (0..n)
                    .map(|i| i as f32 * r)
                    .map(|r| xyz(r.cos(), r.sin(), 0.0)),
            )
            .chain(iter::once(xyz(1.0, 0.0, 0.0)))
            .flatten()
            .collect(),
    )
}

fn xyz<T>(x: T, y: T, z: T) -> impl Iterator<Item = T>
{
    iter::once(x).chain(iter::once(y)).chain(iter::once(z))
}
