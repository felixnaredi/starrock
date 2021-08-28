use getset::Getters;

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
        let x = &mut self.position[0];
        *x += self.velocity[0];
        if *x > 1. {
            *x = -1.;
        }
        if -1. > *x {
            *x = 1.;
        }

        let y = &mut self.position[1];
        *y += self.velocity[1];
        if *y > 1. {
            *y = -1.;
        }
        if -1. > *y {
            *y = 1.;
        }
    }
}
