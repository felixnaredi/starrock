use std::{
    cell::RefCell,
    f32::consts::PI,
    rc::Rc,
};

use rand::{
    seq::SliceRandom,
    Rng,
};
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::WebGlRenderingContext;

use crate::{
    background::Background,
    context::{
        Context,
        ContextDescriptorBuilder,
    },
    dom,
    keyboard_event_bus::KeyboardEventBus,
    rock::{
        Rock,
        RockDescriptorBuilder,
        RockShape,
    },
    rock_renderer::RockRenderer,
    ship::{
        Ship,
        ShipDescriptorBuilder,
    },
    ship_renderer::{
        ShipRenderer,
        ShipRendererDescriptorBuilder,
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
    let context = init_context().unwrap();

    // ---------------------------------------------------------------------------------------------
    // Initialize background.
    // ---------------------------------------------------------------------------------------------

    let background = Rc::new(RefCell::new(Background::new(&context)?));
    {
        let background = Rc::clone(&background);

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

    // ---------------------------------------------------------------------------------------------
    // Initialize ship.
    // ---------------------------------------------------------------------------------------------

    let ship = Rc::new(RefCell::new(Ship::new(
        &ShipDescriptorBuilder::default()
            .position([2., 3. / 2.])
            .size([0.075, 0.075])
            .yaw(PI / 4.)
            .build()
            .unwrap(),
    )));
    let ship_renderer = ShipRenderer::new(
        &context,
        &ShipRendererDescriptorBuilder::default()
            .tail_x(-1. / 6.)
            .wing_angle(23. / 36. * PI)
            .build()
            .unwrap(),
    )?;

    // ---------------------------------------------------------------------------------------------
    // Initialize rocks.
    // ---------------------------------------------------------------------------------------------

    let mut rng = rand::thread_rng();

    let mut rocks: Vec<_> = (0..11)
        .map(|_| {
            let size = 0.05 + rng.gen_range(0.0, 0.1);
            let position = [rng.gen_range(0., 4.), rng.gen_range(0., 3.)];
            let velocity = [rng.gen_range(-10e-3, 10e-3), rng.gen_range(-10e-3, 10e-3)];
            let shape = [
                RockShape::Pentagon,
                RockShape::Hexagon,
                RockShape::Septagon,
                RockShape::Octagon,
            ]
            .choose(&mut rng)
            .unwrap()
            .clone();

            Rock::new(
                &RockDescriptorBuilder::default()
                    .shape(shape)
                    .size([size, size])
                    .position(position)
                    .velocity(velocity)
                    .build()
                    .unwrap(),
            )
        })
        .collect();

    let rock_renderer = RockRenderer::new(&context)?;

    // ---------------------------------------------------------------------------------------------
    // Setup and start the run loop.
    // ---------------------------------------------------------------------------------------------

    let context = Context::new(
        ContextDescriptorBuilder::default()
            .render_context(context)
            .canvas_width(dom::canvas().unwrap().client_width() as u32)
            .canvas_height(dom::canvas().unwrap().client_height() as u32)
            .foreground_perspective_matrix([
                [2. / 4., 0., 0., 0.],
                [0., 2. / 3., 0., 0.],
                [0., 0., 1., 0.],
                [-1., -1., 0., 1.],
            ])
            .build()
            .map_err(|error| format!("{}", error))?,
    );

    let keyboard_event_bus = KeyboardEventBus::new()?;
    let run_loop = Rc::new(RefCell::new(None));
    let _run_loop = Rc::clone(&run_loop);

    *run_loop.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for key in keyboard_event_bus.keys_held_down() {
            match key {
                'w' => ship.borrow_mut().accelerate_forward(0.0025),
                'a' => ship.borrow_mut().accelerate_yaw_rotation(PI / 77.),
                's' => ship.borrow_mut().accelerate_forward(-0.0025),
                'd' => ship.borrow_mut().accelerate_yaw_rotation(-PI / 77.),
                _ => (),
            }
        }
        let gl = context.render_context();

        gl.clear_color(0.0, 1.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        (*background).borrow().render(&context);

        for rock in rocks.iter_mut() {
            rock.update();
            rock_renderer.render(&context, &rock);
        }

        ship.borrow_mut().update();
        ship_renderer.render(&context, &ship.borrow());

        request_animation_frame(_run_loop.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(run_loop.borrow().as_ref().unwrap());

    Ok(())
}

// -------------------------------------------------------------------------------------------------
// Helper functions.
// -------------------------------------------------------------------------------------------------

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
