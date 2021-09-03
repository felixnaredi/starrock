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

    pub fn hitbox(&self) -> CircularHitbox
    {
        // TODO:
        //   The hitbox for a rock should have the greatest possitble radius that is not outside its polygon.
        let radius = self.size[0].min(self.size[1]);
        CircularHitbox::new(self.position.clone(), radius)
    }
}
