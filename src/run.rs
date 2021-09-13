use std::{
    cell::RefCell,
    collections::HashMap,
    f32::consts::PI,
    rc::Rc,
};

use vecmath::{
    vec2_add,
    vec2_mul,
    vec2_scale,
};
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::WebGlRenderingContext;

use crate::{
    background::Background,
    bullet::{
        Bullet,
        UpdateBulletEvent,
    },
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
    ship::Ship,
    ship_renderer::ShipRenderer,
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
    let mut ship_fire_bullet_countdown = 0;

    let bullet_renderer = BulletRenderer::new(&context)?;

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
                ' ' => {
                    if ship_fire_bullet_countdown == 0 {
                        let ship = ship.borrow();
                        let direction = [ship.yaw().cos(), ship.yaw().sin()];
                        let position =
                            vec2_add(*ship.position(), vec2_mul(direction, *ship.size()));
                        let velocity = vec2_scale(direction, 0.040);

                        bullets.push(
                            Bullet::builder()
                                .position(position)
                                .velocity(velocity)
                                .size([0.0750, 0.0075])
                                .countdown(180)
                                .build()
                                .unwrap(),
                        );

                        ship_fire_bullet_countdown = 30;
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
        let bullet_collision_indices: HashMap<_, Vec<_>> = bullets
            .iter()
            .enumerate()
            .map(|(i, bullet)| {
                (
                    i,
                    rocks
                        .iter()
                        .enumerate()
                        .filter_map(|(j, rock)| {
                            bullet.hitbox().intersects(&rock.hitbox()).map(|_| j)
                        })
                        .collect(),
                )
            })
            .collect();

        for (i, js) in bullet_collision_indices.into_iter() {
            let bullet = &mut bullets[i];

            for j in js {
                let rock = &mut rocks[j];

                bullet.push_collision(Collision::Rock(
                    OtherCollisionObject::builder()
                        .position(bullet.hitbox().intersects(&rock.hitbox()).unwrap())
                        .velocity(rock.velocity().clone())
                        .weight(rock.weight().clone())
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

        if ship_fire_bullet_countdown > 0 {
            ship_fire_bullet_countdown -= 1;
        }

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
