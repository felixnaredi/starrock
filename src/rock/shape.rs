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
        use RockShape::*;

        match self {
            Pentagon => 5,
            Hexagon => 6,
            Septagon => 7,
            Octagon => 8,
        }
    }

    pub fn iter() -> impl Iterator<Item = RockShape>
    {
        use RockShape::*;

        iter::once(Pentagon)
            .chain(iter::once(Hexagon))
            .chain(iter::once(Septagon))
            .chain(iter::once(Octagon))
    }
}

impl Distribution<RockShape> for Standard
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockShape
    {
        let i = rng.gen_range(0..4);

        RockShape::iter()
            .enumerate()
            .find_map(|(j, shape)| (i == j).then(|| shape))
            .unwrap()
    }
}
