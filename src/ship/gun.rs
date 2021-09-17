use vecmath::{
    vec2_add,
    vec2_mul,
    vec2_scale,
};

use crate::{
    bullet::Bullet,
    ship::Ship,
};

#[derive(Builder, Clone, Debug)]
pub struct ShipGun
{
    bullet_speed: f32,
    bullet_duration: u32,
    energy_cost: f32,
    period: u32,

    #[builder(setter(skip), default = "0")]
    period_countdown: u32,
}

impl ShipGun
{
    pub fn builder() -> ShipGunBuilder
    {
        ShipGunBuilder::default()
    }

    pub fn fire(&mut self, energy: &mut f32, ship: &Ship) -> Option<Bullet>
    {
        if *energy > self.energy_cost && self.period_countdown == 0 {
            *energy -= self.energy_cost;
            self.period_countdown = self.period;

            let yaw = ship.yaw();
            let direction = [yaw.cos(), yaw.sin()];
            let position = vec2_add(*ship.position(), vec2_mul(direction, *ship.size()));
            let velocity = vec2_scale(direction, self.bullet_speed);

            Some(
                Bullet::builder()
                    .position(position)
                    .velocity(velocity)
                    .size([0.0750, 0.0125])
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
        if self.period_countdown > 0 {
            self.period_countdown -= 1;
        }
    }
}
