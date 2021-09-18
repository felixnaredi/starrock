use std::f32::consts::PI;

use getset::{
    Getters,
    Setters,
};
use vecmath::{
    vec2_add,
    vec2_sub,
};

use super::shape::RockShape;
use crate::{
    collision::{
        CircularHitbox,
        Collision,
        ElasticCollision,
        ElasticCollisionObject,
    },
    foreground,
};

#[derive(Builder, Clone, Debug)]
pub struct RockDescriptor
{
    shape: RockShape,
    size: [f32; 2],
    position: [f32; 2],
    velocity: [f32; 2],
}

impl RockDescriptor
{
    pub fn builder() -> RockDescriptorBuilder
    {
        RockDescriptorBuilder::default()
    }
}

#[derive(Debug, Getters, Setters)]
pub struct Rock
{
    #[getset(get = "pub")]
    shape: RockShape,

    #[getset(get = "pub")]
    size: [f32; 2],

    #[getset(get = "pub")]
    position: [f32; 2],

    #[getset(get = "pub")]
    velocity: [f32; 2],

    #[getset(get = "pub")]
    collisions: Option<Vec<Collision>>,
}

impl Rock
{
    pub fn new(descriptor: &RockDescriptor) -> Rock
    {
        Rock {
            shape: descriptor.shape.clone(),
            size: descriptor.size,
            position: descriptor.position,
            velocity: descriptor.velocity,
            collisions: Some(Vec::new()),
        }
    }

    pub fn update(&mut self) -> impl Iterator<Item = Collision>
    {
        use Collision::*;

        let collisions = self.collisions.replace(Vec::new()).unwrap();

        self.velocity = collisions
            .iter()
            .map(|collision| match collision {
                Rock(other) | Bullet(other) | Ship(other) => ElasticCollision::builder()
                    .target(self)
                    .other(other)
                    .build()
                    .unwrap()
                    .target_velocity_delta(),
            })
            .fold(self.velocity, |velocity, delta| vec2_sub(velocity, delta));

        self.position = vec2_add(self.position, self.velocity);
        foreground::position_modulo(&mut self.position);

        collisions.into_iter()
    }

    pub fn push_collision(&mut self, collision: Collision)
    {
        self.collisions.as_mut().unwrap().push(collision);
    }

    /// The hitbox of the `Rock`.
    ///
    /// The hitbox of a rock is a circle with its center at the same position as the rock and with the radius being the
    /// greatest radius possible so that the hitbox is still fully inside the polygon of the rock.
    pub fn hitbox(&self) -> CircularHitbox
    {
        let sides = self.shape.sides() as f32;
        let size = self.size[0].min(self.size[1]);
        let rad = 2. * PI / sides;
        let w = 0.5 * (1. + rad.cos()) * size;
        let h = 0.5 * (0. + rad.sin()) * size;
        let radius = (w.powi(2) + h.powi(2)).sqrt();

        CircularHitbox::new(self.position.clone(), radius)
    }

    fn area(&self) -> f32
    {
        let w = self.size[0];
        let h = self.size[1];

        if w == h {
            let sides = self.shape.sides() as f32;
            let angle = 2. * PI / sides;
            w.powi(2) * angle.sin() / 2. * sides
        } else {
            panic!("area for ellipsis not implemented")
        }
    }

    pub fn weight(&self) -> f32
    {
        4. * self.area()
    }
}

impl ElasticCollisionObject for Rock
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
        self.weight()
    }
}
