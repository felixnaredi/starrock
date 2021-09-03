use std::f32::consts::PI;

use getset::{
    Getters,
    Setters,
};
use vecmath::{
    vec2_add,
    vec2_dot,
    vec2_normalized_sub,
    vec2_scale,
    vec2_sub,
};

use crate::{
    collision::{
        CircularHitbox,
        Collision,
    },
    foreground,
    rock_shape::RockShape,
};

#[derive(Builder)]
pub struct RockDescriptor
{
    shape: RockShape,
    size: [f32; 2],
    position: [f32; 2],
    velocity: [f32; 2],
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

    #[getset(get = "pub", set = "pub")]
    collision: Option<Collision>,
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
            collision: None,
        }
    }

    pub fn update(&mut self)
    {
        // TODO:
        //   This is just a placeholder collision.
        self.collision.take().map(|collision| {
            let other_position = collision.other_objects_position().clone();
            let other_velocity = collision.other_objects_velocity().clone();

            let direction = vec2_normalized_sub(other_position, self.position);
            let a1 = vec2_scale(other_velocity, vec2_dot(direction, other_velocity));
            let a2 = vec2_sub(other_velocity, a1);
            let b1 = vec2_scale(self.velocity, vec2_dot(direction, other_velocity));
            self.velocity = vec2_add(a2, b1);
        });
        self.position = vec2_add(self.position, self.velocity);
        foreground::position_modulo(&mut self.position);
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
}
