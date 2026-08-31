use crate::Powercap;

use super::Gene;

impl Powercap {
    /// How well this gene matches the secondary preference of a power limit proportional to `energy_preference`.
    ///
    /// Prefer power limits proportional to (1 - energy_preference / 2)
    /// So maximum power at runtime-oriented, and half power at energy-oriented
    pub fn alignment(&self, energy_preference: f32) -> f32 {
        let target_power = 1.0 - 0.5 * energy_preference;
        1.0 - (self.powercap - target_power).abs().clamp(self.min_power, self.max_power)
    }
}

impl Gene for Powercap {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let powercap = self.powercap * t + other.powercap * (1.0 - t);
        Self { powercap, min_power: self.min_power, max_power: self.max_power }
    }

    fn mutate(&mut self, strength: f32) -> f32 {
        let delta = rand::random_range(-strength..=strength);
        self.powercap = (self.powercap + delta).clamp(self.min_power, self.max_power);
        delta
    }

    fn similarity(&self, other: &Self) -> f32 {
        (self.powercap - other.powercap).abs()
    }
}
