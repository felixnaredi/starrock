use getset::Getters;
use ndarray::arr2;
use wasm_bindgen::JsValue;
use web_sys::{
    WebGlBuffer,
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

        let (width, height) = if (4. / 3.) * canvas_height > canvas_width {
            (canvas_width * (3. / 4.), canvas_height)
        } else {
            (canvas_width, canvas_height * (4. / 3.))
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
            type_,
            format,
            None,
        )?;

        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
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
            vec![1., 1., 0., -1., 1., 0., -1., -1., 0., 1., -1., 0.],
        )?;

        Ok(ForegroundRenderer {
            texture,
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
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        let canvas_width = context.canvas_width().clone() as f32;
        let canvas_height = context.canvas_height().clone() as f32;

        let matrix = if (4. / 3.) * canvas_height > canvas_width {
            let (w, h) = (canvas_width * (3. / 4.), canvas_height);
            [
                [1., 0., 0., 0.],
                [0., w / h, 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        } else {
            let (w, h) = (canvas_width, canvas_height * (4. / 3.));
            [
                [h / w, 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]
        };
        let location = gl.get_uniform_location(&self.program, "perspective_matrix");
        gl.uniform_matrix4fv_with_f32_array(
            location.as_ref(),
            false,
            arr2(&matrix).view().as_slice().unwrap(),
        );

        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);

        gl.use_program(None);
    }
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String>
{
    gl::compile_vertex_shader(
        context,
        r#"
    attribute vec4 position;

    uniform mat4 perspective_matrix;

    varying vec4 vertex_position;

    void main() {
      gl_Position = perspective_matrix * position;
      vertex_position = position;
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

        varying vec4 vertex_position;

    void main() {
        float x1 = 0.0;
        float y1 = 0.0;

        float y0 = (vertex_position.y + 1.0) / 2.0;
        if (y0 < 1.0 / 3.0) {
            y1 = 0.5;
        } else if (y0 < 2.0 / 3.0) {
            y1 = 0.0;
        } else {
            y1 = 0.5;
        }

        float x0 = (vertex_position.x + 1.0) / 2.0;
        if (x0 < 1.0 / 4.0) {
            x1 = 0.5;
        } else if (x0 < 2.0 / 4.0) {
            x1 = 0.0;
        } else if (x0 < 3.0 / 4.0) {
            x1 = 0.5;
        } else {
            x1 = 0.0;
        }

        vec3 rgb = vec3(abs(x1 + y1 - 0.5) * 2.0);
        gl_FragColor = vec4(rgb, 0.25);
    }
    "#,
    )
}
