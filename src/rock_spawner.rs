use std::f32::consts::PI;

use rand::Rng;
use vecmath::vec2_scale;

use crate::rock::{
    RockDescriptor,
    RockDescriptorBuilder,
};

#[derive(Builder, Debug)]
pub struct SpawnRandomizedRocksAnywhere
{
    size_range: (f32, f32),
    speed_range: (f32, f32),
}

impl SpawnRandomizedRocksAnywhere
{
    pub fn builder() -> SpawnRandomizedRocksAnywhereBuilder
    {
        SpawnRandomizedRocksAnywhereBuilder::default()
    }
}

impl Iterator for SpawnRandomizedRocksAnywhere
{
    type Item = RockDescriptor;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut rng = rand::thread_rng();

        let (min_size, max_size) = self.size_range;
        let size = rng.gen_range(min_size..max_size);

        let (min_speed, max_speed) = self.speed_range;
        let speed = rng.gen_range(min_speed..max_speed);
        let direction = rng.gen_range(0. ..PI * 2.);
        let velocity = vec2_scale([direction.cos(), direction.sin()], speed);

        Some(
            RockDescriptorBuilder::default()
                .shape(rng.gen())
                .position([rng.gen_range(0. ..4.), rng.gen_range(0. ..3.)])
                .size([size, size])
                .velocity(velocity)
                .build()
                .unwrap(),
        )
    }
}
