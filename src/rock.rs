use getset::Getters;

use crate::foreground;

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

#[derive(Debug, Getters)]
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
        }
    }

    pub fn update(&mut self)
    {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
        foreground::position_modulo(&mut self.position);
    }
}
