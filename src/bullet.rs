use getset::Getters;
use vecmath::vec2_add;

use crate::foreground;

pub enum UpdateBulletEvent
{
    CountdownFinished,
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

        if self.countdown > 0 {
            self.countdown -= 1;
            None
        } else {
            Some(UpdateBulletEvent::CountdownFinished)
        }
    }
}
