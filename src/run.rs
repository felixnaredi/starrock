use std::{
    cell::{
        RefCell,
        RefMut,
    },
    f32::consts::PI,
    rc::Rc,
};

use rand::Rng;
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::WebGlRenderingContext;

use crate::{
    background::Background,
    dom,
    rock::{
        Rock,
        RockDescriptorBuilder,
    },
    ship::{
        Ship,
        ShipDescriptorBuilder,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let context = init_context().unwrap();

    let background = Rc::new(RefCell::new(Background::new(&context)?));

    let descriptor = ShipDescriptorBuilder::default()
        .tail_x(-1. / 6.)
        .wing_angle((23. / 36.) * PI)
        .position([0.5, 0.5])
        .size([0.075, 0.075])
        .yaw(PI / 4.)
        .build()
        .unwrap();
    let ship = Rc::new(RefCell::new(Ship::new(&context, &descriptor)?));

    {
        let ship = ship.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            RefMut::map(ship.as_ref().borrow_mut(), |ship| {
                match event.key().chars().next() {
                    Some('a') => ship.increase_yaw(PI / 27.),
                    Some('d') => ship.increase_yaw(-PI / 27.),
                    _ => (),
                };

                ship
            });
        }) as Box<dyn FnMut(_)>);

        dom::window()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }

    {
        let background = background.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let canvas = dom::canvas().unwrap();

            let width = canvas.client_width() as f32;
            let mut offset = event.client_x() as f32 - width / 2.;
            offset /= width;
            offset *= -2.;
            background.as_ref().borrow_mut().position[0] = offset;

            let height = canvas.client_height() as f32;
            let mut offset = event.client_y() as f32 - height / 2.;
            offset /= height;
            offset *= 2.;
            background.as_ref().borrow_mut().position[1] = offset;
        }) as Box<dyn FnMut(_)>);

        dom::canvas()?
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }

    let mut rng = rand::thread_rng();

    let rocks: Result<Vec<_>, _> = (0..11)
        .map(|_| {
            let size = 0.05 + rng.gen_range(0.0, 0.1);
            let position = [rng.gen_range(-1., 1.), rng.gen_range(-1., 1.)];

            let descriptor = RockDescriptorBuilder::default()
                .sides(rng.gen_range(5, 11))
                .size([size, size])
                .position(position)
                .build()
                .expect("failed to create RockDescriptor");

            Rock::new(&context, &descriptor)
        })
        .collect();
    let mut rocks = rocks?;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_color(0.0, 1.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        (*background).borrow().draw(&context);

        for rock in rocks.iter_mut() {
            rock.update();
            rock.draw(&context);
        }
        (*ship).borrow().draw(&context);

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
    let context = dom::canvas()?
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    Ok(context)
}
