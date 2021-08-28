use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

pub fn compile_vertex_shader(
    context: &WebGlRenderingContext,
    source: &str,
) -> Result<WebGlShader, String>
{
    compile_shader(context, source, WebGlRenderingContext::VERTEX_SHADER)
}

pub fn compile_fragment_shader(
    context: &WebGlRenderingContext,
    source: &str,
) -> Result<WebGlShader, String>
{
    compile_shader(context, source, WebGlRenderingContext::FRAGMENT_SHADER)
}

fn compile_shader(
    context: &WebGlRenderingContext,
    source: &str,
    shader_type: u32,
) -> Result<WebGlShader, String>
{
    let shader = context
        .create_shader(shader_type)
        .ok_or(String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or(String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String>
{
    let program = context
        .create_program()
        .ok_or(String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or(String::from("Unknown error creating program object")))
    }
}

pub fn make_static_draw_array_buffer_f32(
    context: &WebGlRenderingContext,
    data: Vec<f32>,
) -> Result<WebGlBuffer, String>
{
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertex_array = js_sys::Float32Array::view(&data);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    };

    Ok(buffer)
}
