use std::iter;

use getset::Getters;
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
};

#[derive(Getters)]
pub struct ForegroundRenderer
{
    #[getset(get = "pub")]
    texture: WebGlTexture,

    #[getset(get = "pub")]
    framebuffer: WebGlFramebuffer,

    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl ForegroundRenderer
{
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

        let canvas_width = context.canvas_width().clone() as f32;
        let canvas_height = context.canvas_height().clone() as f32;

        let (width, height) = if (6. / 5.) * canvas_height > canvas_width {
            (canvas_width, canvas_width * (5. / 6.))
        } else {
            (canvas_height * (6. / 5.), canvas_height)
        };

        let level = 0;
        let internal_format = WebGlRenderingContext::RGBA as i32;
        let width = width.floor() as i32;
        let height = height.floor() as i32;
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
        // Create vertex buffer.
        //
        let vertex_buffer = gl::make_static_draw_array_buffer_f32(
            gl,
            vec![
                vertex([-1., -1., 0.], [0., 0.]),
                vertex([5., -1., 0.], [1., 0.]),
                vertex([5., 4., 0.], [1., 1.]),
                vertex([-1., 4., 0.], [0., 1.]),
            ]
            .into_iter()
            .flatten()
            .collect(),
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
        })
    }

    pub fn with_render_target_foreground_texture<F: FnOnce()>(&self, context: &Context, lambda: F)
    {
        let gl = context.render_context();

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&self.framebuffer));

        gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.texture),
            0,
        );

        let canvas_width = context.canvas_width().clone() as f32;
        let canvas_height = context.canvas_height().clone() as f32;

        let (width, height) = if (6. / 5.) * canvas_height > canvas_width {
            (canvas_width, canvas_width * (5. / 6.))
        } else {
            (canvas_height * (6. / 5.), canvas_height)
        };
        gl.viewport(0, 0, width as i32, height as i32);

        lambda();

        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
        gl.viewport(0, 0, canvas_width as i32, canvas_height.clone() as i32);
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
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 20, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 2, WebGlRenderingContext::FLOAT, false, 20, 12);

        //
        // Perspective matrix.
        //
        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        let matrix = context.foreground_perspective_matrix();
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            arr2(matrix).view().as_slice().unwrap(),
        );

        //
        // View matrix.
        //
        let canvas_width = context.canvas_width().clone() as f32;
        let canvas_height = context.canvas_height().clone() as f32;

        let matrix = if (6. / 5.) * canvas_height > canvas_width {
            let (w, h) = (canvas_width * (5. / 6.), canvas_height);
            [
                [1., 0., 0., 0.],
                [0., w / h, 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        } else {
            let (w, h) = (canvas_width, canvas_height * (6. / 5.));
            [
                [h / w, 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        };
        let location = gl.get_uniform_location(&self.program, "view_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            arr2(&matrix).view().as_slice().unwrap(),
        );

        //
        // Draw.
        //
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);

        //
        // Cleanup.
        //
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

    uniform mat4 perspective_matrix;
    uniform mat4 view_matrix;

    varying vec4 vertex_position;
    varying vec2 _texcoord;

    void main() {
      gl_Position = view_matrix * perspective_matrix * vec4(position, 1.0);
      vertex_position = perspective_matrix * vec4(position, 1.0);
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

    varying vec4 vertex_position;
    varying vec2 _texcoord;

    void main() {
        float x0 = (vertex_position.x + 1.0) / 2.0;
        float y0 = (vertex_position.y + 1.0) / 2.0;

        float alpha = 0.4;
        if ((y0 < 1.0 / 5.0 || y0 > 4.0 / 5.0) || (x0 < 1.0 / 6.0 || x0 > 5.0 / 6.0)) {
            alpha = 0.2;
        }
        if ((y0 < 1.0 / 5.0 || y0 > 4.0 / 5.0) && (x0 < 1.0 / 6.0 || x0 > 5.0 / 6.0)) {
            alpha = 0.0;
        }

        float y1 = 0.0;
        if (y0 < 1.0 / 5.0) {
            y1 = 0.5;
        } else if (y0 < 2.0 / 5.0) {
            y1 = 0.0;
        } else if (y0 < 3.0 / 5.0) {
            y1 = 0.5;
        } else if (y0 < 4.0 / 5.0) {
            y1 = 0.0;
        } else {
            y1 = 0.5;
        }
        
        float x1 = 0.0;
        if (x0 < 1.0 / 6.0) {
            x1 = 0.5;
        } else if (x0 < 2.0 / 6.0) {
            x1 = 0.0;
        } else if (x0 < 3.0 / 6.0) {
            x1 = 0.5;
        } else if (x0 < 4.0 / 6.0) {
            x1 = 0.0;
        } else if (x0 < 5.0 / 6.0) {
            x1 = 0.5;
        } else {
            x1 = 0.0;
        }

        float k = abs(x1 + y1 - 0.5) * 2.0;
        vec3 rgb = vec3(0.7 * k + 0.5 * (1.0 - k));
        
        vec4 pixel = texture2D(texture, _texcoord);
        
        if (pixel.a > 0.5) {
            gl_FragColor = pixel;
        } else {
            gl_FragColor = vec4(rgb, alpha);
        }
    }
    "#,
    )
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
