use std::{
    collections::HashMap,
    f32::consts::PI,
    iter,
};

use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

use crate::{
    context::Context,
    gl,
    matrix::{
        Matrix4x4,
        Scale,
        Translate,
    },
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

        //
        // Position and scale the rock with a world matrix.
        //
        let matrix = Scale::id()
            .vec2(rock.size())
            .into_arr2()
            .dot(&Translate::id().vec2(rock.position()).into_arr2());

        let location = gl.get_uniform_location(&self.program, "world_matrix");

        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            matrix.view().as_slice().unwrap(),
        );

        //
        // Setup the projection matrix.
        //
        let location = gl.get_uniform_location(&self.program, "projection_matrix");
        let matrix = context.foreground_projection_matrix().clone().into_array();

        gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &matrix);

        //
        // Draw.
        //
        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLE_FAN,
            0,
            rock.shape().sides() as i32 + 2,
        );

        //
        // Clean-up.
        //
        gl.use_program(None);
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_vertex_shader(
        context,
        r#"
        attribute vec4 position;

        uniform mat4 world_matrix;
        uniform mat4 projection_matrix;

        void main()
        {
            gl_Position = projection_matrix * world_matrix * position;
        }
        "#,
    )
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_fragment_shader(
        context,
        r#"
        void main()
        {
            gl_FragColor = vec4(0.10, 0.10, 0.05, 1.00);
        }
        "#,
    )
}

fn polygon_vertices(n: u32) -> Option<Vec<f32>>
{
    (n > 2).then(|| {
        let r = 2.0 * PI * (1.0 / (n as f32));
        iter::once(xyz(0.0, 0.0, 0.0))
            .chain(
                (0..n)
                    .map(|i| i as f32 * r)
                    .map(|r| xyz(r.cos(), r.sin(), 0.0)),
            )
            .chain(iter::once(xyz(1.0, 0.0, 0.0)))
            .flatten()
            .collect()
    })
}

fn xyz<T>(x: T, y: T, z: T) -> impl Iterator<Item = T>
{
    iter::once(x).chain(iter::once(y)).chain(iter::once(z))
}
