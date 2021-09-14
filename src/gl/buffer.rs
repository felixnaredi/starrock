use getset::Getters;
use js_sys::Uint16Array;
use web_sys::{
    WebGlBuffer,
    WebGlRenderingContext,
};

#[allow(dead_code)]
pub enum BufferUsage
{
    StreamDraw = WebGlRenderingContext::STREAM_DRAW as isize,
    DynamicDraw = WebGlRenderingContext::DYNAMIC_DRAW as isize,
    StaticDraw = WebGlRenderingContext::STATIC_DRAW as isize,
}

pub trait ElementArrayBufferType: Sized
{
    type View: Into<js_sys::Object>;

    unsafe fn view(data: &Vec<Self>) -> Self::View;
}

impl ElementArrayBufferType for u16
{
    type View = Uint16Array;

    unsafe fn view(data: &Vec<Self>) -> Self::View
    {
        Self::View::view(data)
    }
}

#[derive(Debug, Getters)]
pub struct ElementArrayBuffer
{
    buffer: WebGlBuffer,

    #[getset(get = "pub")]
    len: Option<usize>,
}

impl ElementArrayBuffer
{
    pub fn new(gl: &WebGlRenderingContext) -> Result<ElementArrayBuffer, String>
    {
        gl.create_buffer()
            .ok_or(String::from("failed to create ElementArrayBuffer"))
            .map(|buffer| ElementArrayBuffer { buffer, len: None })
    }

    pub fn bind(&self, gl: &WebGlRenderingContext)
    {
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.buffer),
        );
    }

    pub fn unbind(&self, gl: &WebGlRenderingContext)
    {
        gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    pub fn set_data<T: ElementArrayBufferType>(
        &mut self,
        gl: &WebGlRenderingContext,
        usage: BufferUsage,
        data: &Vec<T>,
    )
    {
        self.bind(gl);

        unsafe {
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &T::view(&data).into(),
                usage as u32,
            )
        }
        self.len = Some(data.len());

        self.unbind(gl);
    }
}
