use getset::{
    Getters,
    Setters,
};

use crate::{
    collision::{
        CircularHitbox,
        Collision,
    },
    foreground,
};

#[derive(Builder)]
pub struct ShipDescriptor
{
    size: [f32; 2],
    position: [f32; 2],
    yaw: f32,
}

#[derive(Getters, Setters)]
pub struct Ship
{
    #[getset(get = "pub")]
    size: [f32; 2],

    #[getset(get = "pub")]
    position: [f32; 2],

    #[getset(get = "pub")]
    velocity: [f32; 2],

    #[getset(get = "pub")]
    yaw: f32,

    #[getset(get = "pub")]
    yaw_delta: f32,

    #[getset(get = "pub", set = "pub")]
    collision: Option<Collision>,
}

impl Ship
{
    pub fn new(descriptor: &ShipDescriptor) -> Ship
    {
        Ship {
            position: descriptor.position,
            size: descriptor.size,
            velocity: [0., 0.],
            yaw: descriptor.yaw,
            yaw_delta: 0.,
            collision: None,
        }
    }

    pub fn accelerate_yaw_rotation(&mut self, amount: f32)
    {
        self.yaw_delta += amount;
    }

    pub fn accelerate_forward(&mut self, amount: f32)
    {
        self.velocity[0] += amount * self.yaw.cos();
        self.velocity[1] += amount * self.yaw.sin();
    }

    pub fn update(&mut self)
    {
        // TODO:
        //   This is just a placeholder collision.
        self.collision.take().map(|collision| {
            self.position = [2., 1.5];
            self.velocity = [0., 0.];
        });

        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
        self.yaw += self.yaw_delta;

        self.velocity[0] *= 0.91;
        self.velocity[1] *= 0.91;
        self.yaw_delta *= 0.45;

        foreground::position_modulo(&mut self.position);
    }

    pub fn hitbox(&self) -> CircularHitbox
    {
        // TODO:
        //   Configure this hitbox to be more fitting.
        let radius = self.size[0].min(self.size[1]);
        CircularHitbox::new(self.position.clone(), radius)
    }
}
