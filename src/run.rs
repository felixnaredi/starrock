use std::{
    cell::RefCell,
    collections::HashMap,
    f32::consts::PI,
    rc::Rc,
};

use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::WebGlRenderingContext;

use crate::{
    background::Background,
    bullet::UpdateBulletEvent,
    bullet_renderer::BulletRenderer,
    collision::{
        Collision,
        OtherCollisionObject,
    },
    context::{
        Context,
        ContextDescriptorBuilder,
    },
    dom,
    foreground_renderer::ForegroundRenderer,
    keyboard_event_bus::KeyboardEventBus,
    matrix::OrthographicProjection,
    rock::{
        Rock,
        UpdateRockEvent,
    },
    rock_renderer::RockRenderer,
    rock_spawner::SpawnRandomizedRocksAnywhere,
    run_loop::RunLoop,
    ship::{
        Ship,
        ShipBoost,
        ShipController,
        ShipGun,
        ShipRenderer,
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
    let context = context().unwrap();

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

    let ship = Rc::new(RefCell::new(
        Ship::builder()
            .position([2., 3. / 2.])
            .size([0.075, 0.075])
            .weight(5. * 10e-3)
            .yaw(PI / 4.)
            .tail_x(-1. / 9.)
            .wing_angle(23. / 36. * PI)
            .build()
            .map_err(|error| format!("{}", error))?,
    ));

    let mut ship_controller = ShipController::builder()
        .ship(Rc::downgrade(&ship))
        .forward_acceleration(0.0025)
        .backward_acceleration(0.0015)
        .yaw_acceleration(PI / 77.)
        .energy_max(100.)
        .energy_regeneracy(0.5)
        .boost(
            ShipBoost::builder()
                .multiplier(2.5)
                .cost(3.)
                .build()
                .map_err(|error| format!("{}", error))?,
        )
        .gun(
            ShipGun::builder()
                .bullet_duration(120)
                .bullet_speed(0.05)
                .energy_cost(15.)
                .period(15)
                .build()
                .map_err(|error| format!("{}", error))?,
        )
        .build()
        .map_err(|error| format!("{}", error))?;

    let ship_renderer = ShipRenderer::new(&context, &ship.borrow())?;

    // ---------------------------------------------------------------------------------------------
    // Initialize rocks.
    // ---------------------------------------------------------------------------------------------

    let mut rocks: Vec<_> = SpawnRandomizedRocksAnywhere::builder()
        .size_range(0.05..0.15)
        .speed_range(10e-4..1.5 * 10e-3)
        .build()
        .unwrap()
        .take(11)
        .map(|descriptor| Rock::new(&descriptor))
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
            .foreground_projection_matrix(
                OrthographicProjection::default()
                    .abscissa(-1. ..5.)
                    .ordinate(-1. ..4.)
                    .build(),
            )
            .build()
            .map_err(|error| format!("{}", error))?,
    );

    // ---------------------------------------------------------------------------------------------
    // Bullets.
    // ---------------------------------------------------------------------------------------------
    let mut bullets = Vec::new();
    let bullet_renderer = BulletRenderer::new(&context)?;

    // ---------------------------------------------------------------------------------------------
    // Foreground renderer.
    // ---------------------------------------------------------------------------------------------
    let foreground_renderer = ForegroundRenderer::new(&context)?;

    // ---------------------------------------------------------------------------------------------
    // Setup and start the run loop.
    // ---------------------------------------------------------------------------------------------

    let keyboard_event_bus = KeyboardEventBus::new()?;

    let run_loop = RunLoop::new(move || {
        ship_controller.set_boost(keyboard_event_bus.key_is_down('n'));

        for key in keyboard_event_bus.keys_held_down() {
            match key {
                'w' => ship_controller.thrust_forward(),
                'a' => ship_controller.steer_counter_clockwise(),
                's' => ship_controller.thrust_backwards(),
                'd' => ship_controller.steer_clockwise(),
                ' ' => {
                    if let Some(bullet) = ship_controller.fire_bullet() {
                        bullets.push(bullet);
                    }
                }
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
                            if i != j {
                                rock.hitbox()
                                    .intersects(&other.hitbox())
                                    .map(|position| (j, position))
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            })
            .collect();

        for (i, js) in rock_collision_map.iter() {
            for (j, position) in js.iter() {
                let other = &rocks[*j];
                let other = OtherCollisionObject::builder()
                    .position(position.clone())
                    .velocity(other.velocity().clone())
                    .weight(other.weight())
                    .build()
                    .unwrap();

                let rock = &mut rocks[*i];
                rock.push_collision(Collision::Rock(other));
            }
        }

        //
        // Check if ship has collided with rocks.
        //
        let hitbox = ship.borrow().hitbox();

        for rock in rocks.iter_mut() {
            if let Some(position) = hitbox.intersects(&rock.hitbox()) {
                ship.borrow_mut().push_collision(Collision::Rock(
                    OtherCollisionObject::builder()
                        .position(position)
                        .velocity(rock.velocity().clone())
                        .weight(rock.weight())
                        .build()
                        .unwrap(),
                ));
            }

            if let Some(position) = rock.hitbox().intersects(&hitbox) {
                let ship = ship.borrow();
                rock.push_collision(Collision::Ship(
                    OtherCollisionObject::builder()
                        .position(position)
                        .velocity(ship.velocity().clone())
                        .weight(*ship.weight())
                        .build()
                        .unwrap(),
                ));
            }
        }

        //
        // Check if bullets has collided with rocks.
        //
        for bullet in bullets.iter_mut() {
            for rock in rocks.iter_mut() {
                if let Some(position) = bullet.hitbox().intersects(&rock.hitbox()) {
                    bullet.push_collision(Collision::Rock(
                        OtherCollisionObject::builder()
                            .position(position)
                            .velocity(rock.velocity().clone())
                            .weight(rock.weight())
                            .build()
                            .unwrap(),
                    ));
                    rock.push_collision(Collision::Bullet(
                        OtherCollisionObject::builder()
                            .position(rock.hitbox().intersects(&bullet.hitbox()).unwrap())
                            .velocity(bullet.velocity().clone())
                            .weight(0.)
                            .build()
                            .unwrap(),
                    ));
                }
            }
        }

        //
        // Update state.
        //
        let mut countdown_finished = Vec::new();
        let mut hit_by_rock = Vec::new();

        for (i, bullet) in bullets.iter_mut().enumerate() {
            match bullet.update() {
                Some(UpdateBulletEvent::CountdownFinished) => countdown_finished.push(i),
                Some(UpdateBulletEvent::HitByRock) => hit_by_rock.push(i),
                _ => (),
            }
        }

        for i in countdown_finished
            .into_iter()
            .chain(hit_by_rock.into_iter())
        {
            bullets.remove(i);
        }

        let mut rocks_hit_by_bullets = Vec::new();

        for (i, rock) in rocks.iter_mut().enumerate() {
            match rock.update() {
                Some(UpdateRockEvent::HitByBullet) => rocks_hit_by_bullets.push(i),
                _ => (),
            }
        }
        for i in rocks_hit_by_bullets.into_iter() {
            rocks.remove(i);
        }

        ship.borrow_mut().update();
        ship_controller.update();

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

            for rock in rocks.iter() {
                rock_renderer.render(&context, rock);
            }

            for bullet in bullets.iter() {
                bullet_renderer.render(&context, bullet);
            }

            ship_renderer.render(&context, &ship.borrow());
        });
        foreground_renderer.render(&context);
    });

    run_loop.start();

    Ok(())
}

// -------------------------------------------------------------------------------------------------
// Helper functions.
// -------------------------------------------------------------------------------------------------

fn context() -> Result<WebGlRenderingContext, JsValue>
{
    let context = dom::canvas()?
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    Ok(context)
}
