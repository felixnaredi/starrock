use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::{
    Document,
    HtmlCanvasElement,
    Window,
};

pub fn window() -> Window
{
    web_sys::window().expect("no global 'window' exists")
}

pub fn document() -> Document
{
    window().document().unwrap()
}

pub fn canvas() -> Result<HtmlCanvasElement, JsValue>
{
    let document = document();
    Ok(document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?)
}
