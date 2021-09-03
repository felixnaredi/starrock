use std::f32::consts::PI;

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RockShape
{
    Pentagon,
    Hexagon,
    Septagon,
    Octagon,
}

impl RockShape
{
    fn sides(&self) -> u32
    {
        match self {
            RockShape::Pentagon => 5,
            RockShape::Hexagon => 6,
            RockShape::Septagon => 7,
            RockShape::Octagon => 8,
        }
    }
}

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
            self.velocity[0] *= -1.;
            self.velocity[1] *= -1.;
        });
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
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
