use getset::Getters;
use vecmath::vec2_add;

use crate::{
    collision::{
        CircularHitbox,
        Collision,
    },
    foreground,
};

pub enum UpdateBulletEvent
{
    CountdownFinished,
    HitByRock,
}

#[derive(Builder, Clone, Debug, Getters)]
pub struct Bullet
{
    #[getset(get = "pub")]
    position: [f32; 2],

    #[getset(get = "pub")]
    velocity: [f32; 2],

    #[getset(get = "pub")]
    size: [f32; 2],

    #[getset(get = "pub")]
    countdown: u32,

    #[getset(get = "pub")]
    #[builder(default = "Vec::new()")]
    collisions: Vec<Collision>,
}

impl Bullet
{
    pub fn builder() -> BulletBuilder
    {
        BulletBuilder::default()
    }

    pub fn update(&mut self) -> Option<UpdateBulletEvent>
    {
        self.position = vec2_add(self.position, self.velocity);
        foreground::position_modulo(&mut self.position);

        if self
            .collisions
            .iter()
            .any(|collision| matches!(collision, Collision::Rock(_)))
        {
            Some(UpdateBulletEvent::HitByRock)
        } else if self.countdown < 1 {
            Some(UpdateBulletEvent::CountdownFinished)
        } else {
            self.countdown -= 1;
            self.collisions.clear();
            None
        }
    }

    pub fn push_collision(&mut self, collision: Collision)
    {
        self.collisions.push(collision);
    }

    pub fn hitbox(&self) -> CircularHitbox
    {
        CircularHitbox::new(self.position.clone(), self.size[0])
    }
}
