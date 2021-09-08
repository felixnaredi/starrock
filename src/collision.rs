use getset::Getters;

#[derive(Builder, Debug, Getters)]
pub struct Collision
{
    #[get = "pub"]
    other_objects_position: [f32; 2],

    #[get = "pub"]
    other_objects_velocity: [f32; 2],

    #[get = "pub"]
    other_objects_weight: f32,
}

impl Collision
{
    pub fn builder() -> CollisionBuilder
    {
        CollisionBuilder::default()
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

    pub fn intersects(&self, other: &CircularHitbox) -> Option<[f32; 2]>
    {
        let (x1, y1) = (self.position[0], self.position[1]);

        let (x2, y2) = (other.position[0], other.position[1]);

        let x2 = match x2 - x1 {
            delta if delta < -2. => x2 + 4.,
            delta if delta > 2. => x2 - 4.,
            _ => x2,
        };
        let y2 = match y2 - y1 {
            delta if delta < -1.5 => y2 + 3.,
            delta if delta > 1.5 => y2 - 3.,
            _ => y2,
        };
        let x = x2 - x1;
        let y = y2 - y1;
        (self.radius + other.radius > (x * x + y * y).sqrt()).then(|| [x2, y2])
    }
}
