use web_sys::{
    Document,
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
