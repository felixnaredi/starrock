use getset::Getters;
use wasm_bindgen::JsValue;
use web_sys::{
    WebGlRenderingContext,
    WebGlTexture,
};

const TEXTURE_2D: u32 = WebGlRenderingContext::TEXTURE_2D;

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
            Repeat => WebGlRenderingContext::REPEAT,
            ClampToEdge => WebGlRenderingContext::CLAMP_TO_EDGE,
            MirroredRepeat => WebGlRenderingContext::MIRRORED_REPEAT,
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
            Linear => WebGlRenderingContext::LINEAR,
            Nearest => WebGlRenderingContext::NEAREST,
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
            Linear => WebGlRenderingContext::LINEAR,
            Nearest => WebGlRenderingContext::NEAREST,
            NearestMipmapNearest => WebGlRenderingContext::NEAREST_MIPMAP_NEAREST,
            LinearMipmapNearest => WebGlRenderingContext::LINEAR_MIPMAP_NEAREST,
            NearestMipmapLinear => WebGlRenderingContext::NEAREST_MIPMAP_LINEAR,
            LinearMipmapLinear => WebGlRenderingContext::LINEAR_MIPMAP_LINEAR,
        }
    }
}

#[derive(Debug, Builder)]
#[builder(build_fn(skip))]
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
    min_filter: TextureMinificationFilter,
    mag_filter: TextureMagnificationFilter,
    wrap_s: TextureWrappingFunction,
    wrap_t: TextureWrappingFunction,
    pixels: Option<js_sys::Object>
}

impl<'a> Texture2DSpecificationBuilder<'a>
{
    pub fn update(self, gl: &WebGlRenderingContext) -> Result<(), JsValue>
    {
        let texture = self
            .texture
            .ok_or("Texture2DSpecification missing parameter 'texture'")?;
        texture.bind(gl);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            TEXTURE_2D,
            self.level
                .ok_or("Texture2DSpecification missing parameter 'level'")?,
            self.internal_format
                .ok_or("Texture2DSpecification missing parameter 'internal_format'")?,
            self.width
                .ok_or("Texture2DSpecification missing parameter 'width'")?,
            self.height
                .ok_or("Texture2DSpecification missing parameter 'height'")?,
            self.border
                .ok_or("Texture2DSpecification missing parameter 'border'")?,
            self.format
                .ok_or("Texture2DSpecification missing parameter 'format'")?,
            self.type_
                .ok_or("Texture2DSpecification missing parameter 'type_'")?,
            self.pixels.flatten().as_ref(),
        )?;

        gl.tex_parameteri(
            TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            self.min_filter
                .ok_or("Texture2DSpecification missing parameter 'min_filter'")?
                .value() as i32,
        );
        gl.tex_parameteri(
            TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            self.mag_filter
                .ok_or("Texture2DSpecification missing parameter 'mag_filter'")?
                .value() as i32,
        );
        gl.tex_parameteri(
            TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            self.wrap_s
                .ok_or("Texture2DSpecification missing parameter 'wrap_s'")?
                .value() as i32,
        );
        gl.tex_parameteri(
            TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            self.wrap_t
                .ok_or("Texture2DSpecification missing parameter 'wrap_t'")?
                .value() as i32,
        );

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
        gl.bind_texture(TEXTURE_2D, Some(&self.texture));
    }

    pub fn unbind(&self, gl: &WebGlRenderingContext)
    {
        gl.bind_texture(TEXTURE_2D, None);
    }

    pub fn specification(&mut self) -> Texture2DSpecificationBuilder
    {
        Texture2DSpecificationBuilder::default().texture(self)
    }
}
