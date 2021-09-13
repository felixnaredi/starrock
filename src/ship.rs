use getset::{
    Getters,
    Setters,
};
use vecmath::{
    vec2_add,
    vec2_scale,
    vec2_sub,
};

use crate::{
    collision::{
        CircularHitbox,
        Collision,
        ElasticCollision,
        ElasticCollisionObject,
    },
    foreground,
};

#[derive(Builder, Getters, Setters)]
pub struct Ship
{
    #[getset(get = "pub")]
    size: [f32; 2],

    #[getset(get = "pub")]
    position: [f32; 2],

    #[getset(get = "pub")]
    weight: f32,

    #[getset(get = "pub")]
    yaw: f32,

    #[getset(get = "pub")]
    tail_x: f32,

    #[getset(get = "pub")]
    wing_angle: f32,

    #[getset(get = "pub")]
    #[builder(default = "[0., 0.]")]
    velocity: [f32; 2],

    #[getset(get = "pub")]
    #[builder(default = "0.")]
    yaw_delta: f32,

    #[getset(get = "pub")]
    #[builder(default = "Vec::new()")]
    collisions: Vec<Collision>,
}

impl Ship
{
    pub fn builder() -> ShipBuilder
    {
        ShipBuilder::default()
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
        self.velocity = self
            .collisions
            .iter()
            .map(|collision| match collision {
                Collision::Rock(other) | Collision::Bullet(other) | Collision::Ship(other) => {
                    ElasticCollision::builder()
                        .target(self)
                        .other(other)
                        .build()
                        .unwrap()
                        .target_velocity_delta()
                }
            })
            .fold(self.velocity, |velocity, delta| vec2_sub(velocity, delta));

        self.position = vec2_add(self.position, self.velocity);
        foreground::position_modulo(&mut self.position);
        self.yaw += self.yaw_delta;

        self.velocity = vec2_scale(self.velocity, 0.91);
        self.yaw_delta *= 0.45;

        self.collisions.clear();
    }

    pub fn hitbox(&self) -> CircularHitbox
    {
        // TODO:
        //   Configure this hitbox to be more fitting.
        let radius = self.size[0].min(self.size[1]);
        CircularHitbox::new(self.position.clone(), radius)
    }

    pub fn push_collision(&mut self, collision: Collision)
    {
        self.collisions.push(collision);
    }
}

impl ElasticCollisionObject for Ship
{
    fn position(&self) -> [f32; 2]
    {
        self.position
    }

    fn velocity(&self) -> [f32; 2]
    {
        self.velocity
    }

    fn weight(&self) -> f32
    {
        *self.weight()
    }
}
