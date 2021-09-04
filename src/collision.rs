use getset::Getters;

#[derive(Debug, Getters)]
pub struct Collision
{
    #[get = "pub"]
    other_objects_position: [f32; 2],

    #[get = "pub"]
    other_objects_velocity: [f32; 2],
}

impl Collision
{
    pub fn new(other_objects_position: [f32; 2], other_objects_velocity: [f32; 2]) -> Collision
    {
        Collision {
            other_objects_position,
            other_objects_velocity,
        }
    }
}

#[derive(Debug)]
pub struct CircularHitbox
{
    position: [f32; 2],
    radius: f32,
}

impl CircularHitbox
{
    pub fn new(position: [f32; 2], radius: f32) -> CircularHitbox
    {
        CircularHitbox { position, radius }
    }

    pub fn intersects(&self, other: CircularHitbox) -> bool
    {
        let (x1, y1) = (self.position[0], self.position[1]);

        let (x2, y2) = (other.position[0], other.position[1]);

        let x = match x2 - x1 {
            delta if delta < -2. => delta + 4.,
            delta if delta > 2. => delta - 4.,
            delta => delta,
        };
        let y = match y2 - y1 {
            delta if delta < -1.5 => delta + 3.,
            delta if delta > 1.5 => delta - 3.,
            delta => delta,
        };
        self.radius + other.radius > (x * x + y * y).sqrt()
    }
}
