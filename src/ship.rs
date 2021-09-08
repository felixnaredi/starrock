use getset::{
    Getters,
    Setters,
};
use vecmath::{
    vec2_dot,
    vec2_scale,
    vec2_sub,
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
    collisions: Option<Vec<Collision>>,
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
            collisions: None,
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
        if let Some(collisions) = self.collisions.take() {
            for collision in collisions {
                let x_a = self.position;
                let u_a = self.velocity;
                let m_a = self.weight();

                let x_b = collision.other_objects_position().clone();
                let u_b = collision.other_objects_velocity().clone();
                let m_b = *collision.other_objects_weight();

                let dx = vec2_sub(x_a, x_b);
                let nx = dx[0].powi(2) + dx[1].powi(2);
                let du = vec2_sub(u_a, u_b);
                let dot_ux = vec2_dot(du, dx);
                let m = 2. * m_b / (m_a + m_b);

                self.velocity = vec2_sub(u_a, vec2_scale(dx, m * dot_ux / nx));
            }
        };

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

    pub fn weight(&self) -> f32
    {
        1. * self.size[0].min(self.size[1])
    }

    pub fn push_collision(&mut self, collision: Collision)
    {
        let collisions = self.collisions.get_or_insert(Vec::new());
        collisions.push(collision)
    }
}
