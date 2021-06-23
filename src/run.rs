use std::{
    cell::RefCell,
    rc::Rc,
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement,
    WebGlRenderingContext,
};

use crate::{
    background::Background,
    dom,
    rock::{
        Rock,
        RockDescriptorBuilder,
    },
    ship::Ship,
};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let context = init_context().unwrap();

    let background = Background::new(&context)?;
    let ship = Ship::new(&context)?;

    let rocks: Result<Vec<_>, _> = (0..3)
        .map(|i| {
            let descriptor = RockDescriptorBuilder::default()
                .sides(5 + i)
                .size([0.05 + 0.1 * i as f32, 0.05 + 0.1 * i as f32])
                .position([-0.1 + 0.15 * (i * i) as f32, 0.1 + -0.15 * (i * i) as f32])
                .build()
                .expect("failed to create RockDescriptor");
            Rock::new(&context, &descriptor)
        })
        .collect();
    let mut rocks = rocks?;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_color(0.0, 1.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        background.draw(&context);

        for rock in rocks.iter_mut() {
            rock.update();
            rock.draw(&context);
        }
        ship.draw(&context);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>)
{
    dom::window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn init_context() -> Result<WebGlRenderingContext, JsValue>
{
    let document = dom::document();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    Ok(context)
}
