#[derive(Builder, Clone, Debug)]
pub struct ShipBoost
{
    multiplier: f32,
    cost: f32,

    #[builder(setter(skip), default = "false")]
    enabled: bool,

    #[builder(setter(skip), default = "false")]
    active: bool,
}

impl ShipBoost
{
    pub fn builder() -> ShipBoostBuilder
    {
        ShipBoostBuilder::default()
    }

    /// The speed multiplier for active boost.
    ///
    /// If boost is possible the cost of boosting one frame is subtracted from `energy` and the
    /// multiplier is returned.
    ///
    /// If there isn't enough energy to perform the boost it will be disabled until the boost is set
    /// to false. This way, if a player hold down the boost button past the energy limit it will not
    /// be continouosly depleted.
    pub fn multiplier(&mut self, energy: &mut f32) -> Option<f32>
    {
        if self.active && self.enabled {
            if *energy > self.cost {
                *energy -= self.cost;
                Some(self.multiplier)
            } else {
                self.enabled = false;
                None
            }
        } else {
            None
        }
    }

    /// Set the active state of the boost.
    pub fn set(&mut self, state: bool)
    {
        self.active = state;
        if !self.enabled {
            self.enabled = !state;
        }
    }
}
