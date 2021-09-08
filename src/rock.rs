use std::f32::consts::PI;

use getset::{
    Getters,
    Setters,
};
use vecmath::{
    vec2_add,
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
            collisions: None,
        }
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

        self.position = vec2_add(self.position, self.velocity);
        foreground::position_modulo(&mut self.position);
    }

    pub fn push_collision(&mut self, collision: Collision)
    {
        let collisions = self.collisions.get_or_insert(Vec::new());
        collisions.push(collision)
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
