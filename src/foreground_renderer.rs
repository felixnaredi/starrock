use std::iter;

use ndarray::arr2;
use wasm_bindgen::JsValue;
use web_sys::{
    WebGlBuffer,
    WebGlFramebuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
    WebGlTexture,
};

use crate::{
    context::Context,
    gl,
    matrix::{
        self,
        OrthographicProjectionMatrix,
    },
};

/// Renders the foreground.
pub struct ForegroundRenderer
{
    texture: WebGlTexture,
    framebuffer: WebGlFramebuffer,
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
}

impl ForegroundRenderer
{
    /// Creates a new `ForegroundRenderer`.
    pub fn new(context: &Context) -> Result<ForegroundRenderer, JsValue>
    {
        let gl = context.render_context();

        //
        // Create the program for rendering the foreground texture.
        //
        let program = gl::link_program(gl, &vertex_shader(gl)?, &fragment_shader(gl)?)?;

        //
        // Create and setup texture.
        //
        let texture = gl.create_texture().ok_or("failed to create texture")?;
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

        let level = 0;
        let internal_format = WebGlRenderingContext::RGBA as i32;
        let (width, height) = calculate_texture_size(context);
        let border = 0;
        let type_ = WebGlRenderingContext::UNSIGNED_BYTE;
        let format = WebGlRenderingContext::RGBA;
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGlRenderingContext::TEXTURE_2D,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            type_,
            None,
        )?;

        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        //
        // Create vertex buffer and index buffer.
        //
        let vertex_buffer = gl::make_static_draw_array_buffer_f32(
            gl,
            vec![
                //  0,  1,  2,  3,
                vertex([0., 0., 0.], [1. / 6., 1. / 5.]),
                vertex([4., 0., 0.], [5. / 6., 1. / 5.]),
                vertex([4., 3., 0.], [5. / 6., 4. / 5.]),
                vertex([0., 3., 0.], [1. / 6., 4. / 5.]),
                //  4,  5,  6,  7,
                vertex([0., 0., 0.], [5. / 6., 1. / 5.]),
                vertex([1., 0., 0.], [6. / 6., 1. / 5.]),
                vertex([1., 3., 0.], [6. / 6., 4. / 5.]),
                vertex([0., 3., 0.], [5. / 6., 4. / 5.]),
                //  8,  9, 10, 11,
                vertex([3., 0., 0.], [0. / 6., 1. / 5.]),
                vertex([4., 0., 0.], [1. / 6., 1. / 5.]),
                vertex([4., 3., 0.], [1. / 6., 4. / 5.]),
                vertex([3., 3., 0.], [0. / 6., 4. / 5.]),
                // 12, 13, 14, 15,
                vertex([0., 0., 0.], [1. / 6., 4. / 5.]),
                vertex([4., 0., 0.], [5. / 6., 4. / 5.]),
                vertex([4., 1., 0.], [5. / 6., 5. / 5.]),
                vertex([0., 1., 0.], [1. / 6., 5. / 5.]),
                // 16, 17, 18, 19,
                vertex([0., 2., 0.], [1. / 6., 0. / 5.]),
                vertex([4., 2., 0.], [5. / 6., 0. / 5.]),
                vertex([4., 3., 0.], [5. / 6., 1. / 5.]),
                vertex([0., 3., 0.], [1. / 6., 1. / 5.]),
                // 20, 21, 22, 23,
                vertex([3., 2., 0.], [0. / 6., 0. / 5.]),
                vertex([4., 2., 0.], [1. / 6., 0. / 5.]),
                vertex([4., 3., 0.], [1. / 6., 1. / 5.]),
                vertex([3., 3., 0.], [0. / 6., 1. / 5.]),
                // 24, 25, 26, 27,
                vertex([0., 2., 0.], [5. / 6., 0. / 5.]),
                vertex([1., 2., 0.], [6. / 6., 0. / 5.]),
                vertex([1., 3., 0.], [6. / 6., 1. / 5.]),
                vertex([0., 3., 0.], [5. / 6., 1. / 5.]),
                // 28, 29, 30, 31,
                vertex([0., 0., 0.], [5. / 6., 4. / 5.]),
                vertex([1., 0., 0.], [6. / 6., 4. / 5.]),
                vertex([1., 1., 0.], [6. / 6., 5. / 5.]),
                vertex([0., 1., 0.], [5. / 6., 5. / 5.]),
                // 32, 33, 34, 35,
                vertex([3., 0., 0.], [0. / 6., 4. / 5.]),
                vertex([4., 0., 0.], [1. / 6., 4. / 5.]),
                vertex([4., 1., 0.], [1. / 6., 5. / 5.]),
                vertex([3., 1., 0.], [0. / 6., 5. / 5.]),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )?;

        let index_buffer = gl::make_static_draw_element_array_buffer_i16(
            gl,
            vec![
                0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15,
                16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23, 24, 25, 26, 24, 26, 27, 28, 29, 30,
                28, 30, 31, 32, 33, 34, 32, 34, 35,
            ],
        )?;

        //
        // Create framebuffer.
        //
        let framebuffer = gl
            .create_framebuffer()
            .ok_or("failed to create framebuffer")?;

        Ok(ForegroundRenderer {
            texture,
            framebuffer,
            program,
            vertex_buffer,
            index_buffer,
        })
    }

    /// Performs `lambda` with the foreground texture as render target.
    pub fn with_render_target_foreground_texture<F: FnOnce()>(&self, context: &Context, lambda: F)
    {
        let gl = context.render_context();

        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.texture),
            0,
        );

        let (width, height) = calculate_texture_size(context);
        gl.viewport(0, 0, width as i32, height as i32);

        lambda();

        //
        // Cleanup.
        //

        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

        // Restore viewport
        //
        // TODO:
        //   The context could probably keep track of a stack of viewport sizes so that it would be possible to draw
        //   into several textures.
        gl.viewport(
            0,
            0,
            context.canvas_width().clone() as i32,
            context.canvas_height().clone() as i32,
        );
    }

    pub fn render(&self, context: &Context)
    {
        let gl = context.render_context();

        gl.use_program(Some(&self.program));

        //
        // Projection matrix.
        //
        let location = gl.get_uniform_location(&self.program, "projection_matrix");
        let matrix = OrthographicProjectionMatrix::builder()
            .abscissa(0. ..4.)
            .ordinate(0. ..3.)
            .build();
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            arr2(&matrix).view().as_slice().unwrap(),
        );

        //
        // View matrix.
        //
        let canvas_width = context.canvas_width().clone() as f32;
        let canvas_height = context.canvas_height().clone() as f32;

        let matrix = if (4. / 3.) * canvas_height > canvas_width {
            let (w, h) = (canvas_width * (3. / 4.), canvas_height);
            matrix::scale_xy(1., w / h)
        } else {
            let (w, h) = (canvas_width, canvas_height * (4. / 3.));
            matrix::scale_xy(h / w, 1.)
        };
        let location = gl.get_uniform_location(&self.program, "view_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            arr2(&matrix).view().as_slice().unwrap(),
        );

        //
        // Setup vertex buffer.
        //
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 20, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 2, WebGlRenderingContext::FLOAT, false, 20, 12);

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        //
        // Draw.
        //
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer),
        );

        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            54,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        //
        // Clean-up.
        //
        gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
        gl.use_program(None);
    }
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_vertex_shader(
        context,
        r#"
        attribute vec3 position;
        attribute vec2 texcoord;

        uniform mat4 projection_matrix;
        uniform mat4 view_matrix;

        varying vec2 _texcoord;

        void main()
        {
            vec4 p0 = view_matrix * projection_matrix * vec4(position, 1.0);

            gl_Position = p0;
            _texcoord = texcoord;
        }
        "#,
    )
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_fragment_shader(
        context,
        r#"
        precision mediump float;

        uniform sampler2D texture;

        varying vec2 _texcoord;

        void main()
        {
            gl_FragColor = texture2D(texture, _texcoord);
        }
        "#,
    )
}

fn calculate_texture_size(context: &Context) -> (i32, i32)
{
    let canvas_width = context.canvas_width().clone() as f32;
    let canvas_height = context.canvas_height().clone() as f32;

    let (width, height) = if (6. / 5.) * canvas_height > canvas_width {
        (canvas_width, canvas_width * 5. / 6.)
    } else {
        (canvas_height * 6. / 5., canvas_height)
    };

    (width.round() as i32, height.round() as i32)
}

fn vertex(position: [f32; 3], texcoord: [f32; 2]) -> impl Iterator<Item = f32>
{
    xyz(position[0], position[1], position[2]).chain(xy(texcoord[0], texcoord[1]))
}

fn xy<T>(x: T, y: T) -> impl Iterator<Item = T>
{
    iter::once(x).chain(iter::once(y))
}

fn xyz<T>(x: T, y: T, z: T) -> impl Iterator<Item = T>
{
    iter::once(x).chain(iter::once(y)).chain(iter::once(z))
}
