use std::{
    cell::RefCell,
    rc::Weak,
};

use vecmath::{
    vec2_add,
    vec2_mul,
    vec2_scale,
};

use crate::{
    bullet::Bullet,
    ship::Ship,
};

#[derive(Builder, Debug)]
pub struct ShipController
{
    ship: Weak<RefCell<Ship>>,
    forward_acceleration: f32,
    backward_acceleration: f32,
    yaw_acceleration: f32,
    fire_countdown_duration: u32,
    bullet_speed: f32,
    bullet_duration: u32,

    #[builder(default = "0")]
    fire_countdown: u32,
}

impl ShipController
{
    pub fn builder() -> ShipControllerBuilder
    {
        ShipControllerBuilder::default()
    }

    pub fn thrust_forward(&mut self)
    {
        if let Some(ship) = self.ship.upgrade() {
            ship.borrow_mut()
                .accelerate_forward(self.forward_acceleration);
        }
    }

    pub fn thrust_backwards(&mut self)
    {
        if let Some(ship) = self.ship.upgrade() {
            ship.borrow_mut()
                .accelerate_forward(-self.backward_acceleration);
        }
    }

    pub fn steer_counter_clockwise(&mut self)
    {
        if let Some(ship) = self.ship.upgrade() {
            ship.borrow_mut()
                .accelerate_yaw_rotation(self.yaw_acceleration);
        }
    }

    pub fn steer_clockwise(&mut self)
    {
        if let Some(ship) = self.ship.upgrade() {
            ship.borrow_mut()
                .accelerate_yaw_rotation(-self.yaw_acceleration);
        }
    }

    pub fn fire_bullet(&mut self) -> Option<Bullet>
    {
        if self.fire_countdown == 0 {
            let ship = self.ship.upgrade()?;
            let ship = ship.borrow();

            self.fire_countdown = self.fire_countdown_duration;

            let yaw = ship.yaw();
            let direction = [yaw.cos(), yaw.sin()];
            let position = vec2_add(*ship.position(), vec2_mul(direction, *ship.size()));
            let velocity = vec2_scale(direction, self.bullet_speed);

            Some(
                Bullet::builder()
                    .position(position)
                    .velocity(velocity)
                    .size([0.0750, 0.0075])
                    .countdown(self.bullet_duration)
                    .build()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    pub fn update(&mut self)
    {
        if self.fire_countdown > 0 {
            self.fire_countdown -= 1;
        }
    }
}
