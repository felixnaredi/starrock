use getset::Getters;
use wasm_bindgen::JsValue;
use web_sys::{
    WebGlRenderingContext,
    WebGlTexture,
};

type GL = WebGlRenderingContext;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TextureWrappingFunction
{
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

impl TextureWrappingFunction
{
    fn value(&self) -> u32
    {
        use TextureWrappingFunction::*;

        match self {
            Repeat => GL::REPEAT,
            ClampToEdge => GL::CLAMP_TO_EDGE,
            MirroredRepeat => GL::MIRRORED_REPEAT,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TextureMagnificationFilter
{
    Linear,
    Nearest,
}

impl TextureMagnificationFilter
{
    fn value(&self) -> u32
    {
        use TextureMagnificationFilter::*;

        match self {
            Linear => GL::LINEAR,
            Nearest => GL::NEAREST,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TextureMinificationFilter
{
    Linear,
    Nearest,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl TextureMinificationFilter
{
    fn value(&self) -> u32
    {
        use TextureMinificationFilter::*;

        match self {
            Linear => GL::LINEAR,
            Nearest => GL::NEAREST,
            NearestMipmapNearest => GL::NEAREST_MIPMAP_NEAREST,
            LinearMipmapNearest => GL::LINEAR_MIPMAP_NEAREST,
            NearestMipmapLinear => GL::NEAREST_MIPMAP_LINEAR,
            LinearMipmapLinear => GL::LINEAR_MIPMAP_LINEAR,
        }
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Texture2DSpecification<'a>
{
    texture: &'a Texture2D,
    level: i32,
    internal_format: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    type_: u32,

    #[builder(setter(strip_option), default)]
    min_filter: Option<TextureMinificationFilter>,

    #[builder(setter(strip_option), default)]
    mag_filter: Option<TextureMagnificationFilter>,

    #[builder(setter(strip_option), default)]
    wrap_s: Option<TextureWrappingFunction>,

    #[builder(setter(strip_option), default)]
    wrap_t: Option<TextureWrappingFunction>,

    #[builder(setter(strip_option), default)]
    pixels: Option<js_sys::Object>,
}

impl<'a> Texture2DSpecificationBuilder<'a>
{
    pub fn update(self, gl: &WebGlRenderingContext) -> Result<(), JsValue>
    {
        let specification = self.build().map_err(|error| format!("{}", error))?;

        let texture = specification.texture;
        texture.bind(gl);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            specification.level,
            specification.internal_format,
            specification.width,
            specification.height,
            specification.border,
            specification.format,
            specification.type_,
            specification.pixels.as_ref(),
        )?;

        if let Some(filter) = specification.min_filter {
            gl.tex_parameteri(
                GL::TEXTURE_2D,
                GL::TEXTURE_MIN_FILTER,
                filter.value() as i32,
            );
        }

        if let Some(filter) = specification.mag_filter {
            gl.tex_parameteri(
                GL::TEXTURE_2D,
                GL::TEXTURE_MAG_FILTER,
                filter.value() as i32,
            );
        }

        if let Some(wrapping) = specification.wrap_s {
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, wrapping.value() as i32);
        }

        if let Some(wrapping) = specification.wrap_t {
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, wrapping.value() as i32);
        }

        texture.unbind(gl);
        Ok(())
    }
}

#[derive(Debug, Getters)]
pub struct Texture2D
{
    #[getset(get = "pub")]
    texture: WebGlTexture,
}

impl Texture2D
{
    pub fn new(gl: &WebGlRenderingContext) -> Result<Texture2D, String>
    {
        gl.create_texture()
            .ok_or(String::from("failed to create texture"))
            .map(|texture| Texture2D { texture })
    }

    pub fn bind(&self, gl: &WebGlRenderingContext)
    {
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
    }

    pub fn unbind(&self, gl: &WebGlRenderingContext)
    {
        gl.bind_texture(GL::TEXTURE_2D, None);
    }

    pub fn specification(&mut self) -> Texture2DSpecificationBuilder
    {
        Texture2DSpecificationBuilder::default().texture(self)
    }
}
