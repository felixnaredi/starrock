use getset::Getters;

#[derive(Builder)]
pub struct ShipDescriptor
{
    size: [f32; 2],
    position: [f32; 2],
    yaw: f32,
}

#[derive(Getters)]
pub struct Ship
{
    #[getset(get = "pub")]
    position: [f32; 2],

    #[getset(get = "pub")]
    size: [f32; 2],

    #[getset(get = "pub")]
    yaw: f32,
}

impl Ship
{
    pub fn new(descriptor: &ShipDescriptor) -> Ship
    {
        Ship {
            position: descriptor.position,
            size: descriptor.size,
            yaw: descriptor.yaw,
        }
    }

    pub fn increase_yaw(&mut self, amount: f32)
    {
        self.yaw += amount;
    }

    pub fn move_forward(&mut self, amount: f32)
    {
        self.position[0] += amount * self.yaw.cos();
        self.position[1] += amount * self.yaw.sin();
    }
}
