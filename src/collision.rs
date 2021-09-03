use getset::Getters;

#[derive(Debug, Getters)]
pub struct Collision
{
    #[get = "pub"]
    vector: [f32; 2],
}

impl Collision
{
    pub fn new(vector: [f32; 2]) -> Collision
    {
        Collision { vector }
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
        // TODO:
        //   This should also handle cases where one or more of the objects wraps around the edges.
        let (x1, y1) = (self.position[0], self.position[1]);
        let (x2, y2) = (other.position[0], other.position[1]);
        let x = x2 - x1;
        let y = y2 - y1;
        self.radius + other.radius > (x * x + y * y).sqrt()
    }
}
