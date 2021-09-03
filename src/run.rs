use std::{
    cell::RefCell,
    collections::HashMap,
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
    collision::Collision,
    context::{
        Context,
        ContextDescriptorBuilder,
    },
    dom,
    foreground_renderer::ForegroundRenderer,
    keyboard_event_bus::KeyboardEventBus,
    rock::{
        Rock,
        RockDescriptorBuilder,
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
            //.position([0., 0.])
            .size([0.075, 0.075])
            // .size([0.275, 0.275])
            .yaw(PI / 4.)
            //.yaw(0.)
            .build()
            .unwrap(),
    )));
    let ship_renderer = ShipRenderer::new(
        &context,
        &ShipRendererDescriptorBuilder::default()
            .tail_x(-1. / 9.)
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
            let shape = rng.gen();

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
    // Create context.
    // ---------------------------------------------------------------------------------------------

    let context = Context::new(
        ContextDescriptorBuilder::default()
            .render_context(context)
            .canvas_width(dom::canvas().unwrap().client_width() as u32)
            .canvas_height(dom::canvas().unwrap().client_height() as u32)
            .foreground_perspective_matrix({
                let r = 5.;
                let l = -1.;
                let t = 4.;
                let b = -1.;
                [
                    [2. / (r - l), 0., 0., 0.],
                    [0., 2. / (t - b), 0., 0.],
                    [0., 0., 1., 0.],
                    [-(r + l) / (r - l), -(t + b) / (t - b), 0., 1.],
                ]
            })
            .build()
            .map_err(|error| format!("{}", error))?,
    );

    // ---------------------------------------------------------------------------------------------
    // Foreground renderer.
    // ---------------------------------------------------------------------------------------------
    let foreground_renderer = ForegroundRenderer::new(&context)?;

    // ---------------------------------------------------------------------------------------------
    // Setup and start the run loop.
    // ---------------------------------------------------------------------------------------------

    let keyboard_event_bus = KeyboardEventBus::new()?;
    let run_loop = Rc::new(RefCell::new(None));
    let _run_loop = Rc::clone(&run_loop);

    *run_loop.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        for key in keyboard_event_bus.keys_held_down() {
            match key {
                'w' => ship.borrow_mut().accelerate_forward(0.0025),
                'a' => ship.borrow_mut().accelerate_yaw_rotation(PI / 77.),
                's' => ship.borrow_mut().accelerate_forward(-0.0015),
                'd' => ship.borrow_mut().accelerate_yaw_rotation(-PI / 77.),
                _ => (),
            }
        }

        //
        // Check rocks colliding with other rocks.
        //
        let rock_collision_map: HashMap<_, Vec<_>> = rocks
            .iter()
            .enumerate()
            .map(|(i, rock)| {
                (
                    i,
                    rocks
                        .iter()
                        .enumerate()
                        .filter_map(move |(j, other)| {
                            (i != j && rock.hitbox().intersects(other.hitbox())).then(|| j)
                        })
                        .collect(),
                )
            })
            .collect();

        rock_collision_map.iter().for_each(|(i, js)| {
            js.iter().for_each(|j| {
                let velocity = rocks[*j].velocity().clone();
                let rock = &mut rocks[*i];
                rock.set_collision(Some(Collision::new(velocity)));
            });
        });

        //
        // Check if ship has collided with rocks.
        //
        rocks.iter().for_each(|rock| {
            if ship.borrow().hitbox().intersects(rock.hitbox()) {
                ship.borrow_mut()
                    .set_collision(Some(Collision::new([0., 0.])));
            }
        });

        //
        // Update state.
        //
        rocks.iter_mut().for_each(Rock::update);
        ship.borrow_mut().update();

        //
        // Render.
        //
        let gl = context.render_context();

        gl.clear_color(0.0, 1.0, 0.0, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        gl.enable(WebGlRenderingContext::BLEND);
        gl.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        (*background).borrow().render(&context);

        foreground_renderer.with_render_target_foreground_texture(&context, || {
            gl.clear_color(0., 0., 0., 0.);

            // Uncomment to see better how the texture is rendered.
            /*
            gl.clear_color(0., 0., 1., 0.1);
            */

            gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            rocks
                .iter()
                .for_each(|rock| rock_renderer.render(&context, rock));

            ship_renderer.render(&context, &ship.borrow());
        });
        foreground_renderer.render(&context);

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
