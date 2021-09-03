use std::iter;

use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    Rng,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RockShape
{
    Pentagon,
    Hexagon,
    Septagon,
    Octagon,
}

impl RockShape
{
    pub fn sides(&self) -> u32
    {
        match self {
            RockShape::Pentagon => 5,
            RockShape::Hexagon => 6,
            RockShape::Septagon => 7,
            RockShape::Octagon => 8,
        }
    }

    pub fn iter() -> impl Iterator<Item = RockShape>
    {
        iter::once(RockShape::Pentagon)
            .chain(iter::once(RockShape::Hexagon))
            .chain(iter::once(RockShape::Septagon))
            .chain(iter::once(RockShape::Octagon))
    }
}

impl Distribution<RockShape> for Standard
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockShape
    {
        match rng.gen_range(0, 4) {
            0 => RockShape::Pentagon,
            1 => RockShape::Hexagon,
            2 => RockShape::Septagon,
            _ => RockShape::Octagon,
        }
    }
}
