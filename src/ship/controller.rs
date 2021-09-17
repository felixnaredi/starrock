use std::{
    cell::RefCell,
    rc::Weak,
};

use crate::{
    bullet::Bullet,
    ship::{
        Ship,
        ShipBoost,
        ShipGun,
    },
};

#[derive(Builder, Debug)]
pub struct ShipController
{
    ship: Weak<RefCell<Ship>>,
    forward_acceleration: f32,
    backward_acceleration: f32,
    yaw_acceleration: f32,
    energy_max: f32,
    energy_regeneracy: f32,
    boost: ShipBoost,
    gun: ShipGun,

    #[builder(setter(skip), default = "self.energy_max.unwrap()")]
    energy: f32,

    #[builder(setter(skip), default = "1.")]
    boost_multiplier: f32,
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
                .accelerate_forward(self.forward_acceleration * self.boost_multiplier);
        }
    }

    pub fn thrust_backwards(&mut self)
    {
        if let Some(ship) = self.ship.upgrade() {
            ship.borrow_mut()
                .accelerate_forward(-self.backward_acceleration * self.boost_multiplier);
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
        self.ship
            .upgrade()
            .map(|ship| self.gun.fire(&mut self.energy, &ship.borrow()))
            .flatten()
    }

    pub fn set_boost(&mut self, state: bool)
    {
        self.boost.set(state);
        self.boost_multiplier = self.boost.multiplier(&mut self.energy).unwrap_or(1.);
    }

    pub fn update(&mut self)
    {
        if self.energy < self.energy_max {
            self.energy = (self.energy + self.energy_regeneracy).min(self.energy_max);
        }
        self.gun.update();
    }
}
