use getset::Getters;
use vecmath::{
    vec2_dot,
    vec2_scale,
    vec2_sub,
};

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

#[derive(Builder, Debug)]
pub struct ElasticCollision
{
    target_position: [f32; 2],
    target_velocity: [f32; 2],
    target_weight: f32,
    other_position: [f32; 2],
    other_velocity: [f32; 2],
    other_weight: f32,
}

impl ElasticCollision
{
    pub fn builder() -> ElasticCollisionBuilder
    {
        ElasticCollisionBuilder::default()
    }

    pub fn target_velocity_delta(self) -> [f32; 2]
    {
        let dx = vec2_sub(self.target_position, self.other_position);
        let nx = dx[0].powi(2) + dx[1].powi(2);
        let dv = vec2_sub(self.target_velocity, self.other_velocity);

        let m_a = self.target_weight;
        let m_b = self.other_weight;
        let m = 2. * m_b / (m_a + m_b);

        vec2_scale(dx, m * vec2_dot(dv, dx) / nx)
    }
}

pub trait ElasticCollisionObject
{
    fn position(&self) -> [f32; 2];
    fn velocity(&self) -> [f32; 2];
    fn weight(&self) -> f32;
}

impl ElasticCollisionBuilder
{
    pub fn target<T: ElasticCollisionObject>(&mut self, object: &T)
        -> &mut ElasticCollisionBuilder
    {
        self.target_position(object.position());
        self.target_velocity(object.velocity());
        self.target_weight(object.weight());
        self
    }

    pub fn other<T: ElasticCollisionObject>(&mut self, object: &T) -> &mut ElasticCollisionBuilder
    {
        self.other_position(object.position());
        self.other_velocity(object.velocity());
        self.other_weight(object.weight());
        self
    }
}

impl ElasticCollisionObject for Collision
{
    fn position(&self) -> [f32; 2]
    {
        self.other_objects_position
    }

    fn velocity(&self) -> [f32; 2]
    {
        self.other_objects_velocity
    }

    fn weight(&self) -> f32
    {
        self.other_objects_weight
    }
}
