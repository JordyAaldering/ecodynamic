use super::{Gene, lerp};

#[derive(Clone, Debug, PartialEq)]
pub struct PowerGene {
    powercap: f32,
    min_power: f32,
    max_power: f32,
}

impl PowerGene {
    pub fn new(min_power: f32, max_power: f32) -> Self {
        Self { powercap: max_power, min_power, max_power }
    }

    pub fn rand(mut self) -> Self {
        self.powercap = rand::random_range(self.min_power..=self.max_power);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.powercap = lerp(self.min_power, self.max_power, t);
        self
    }

    /// How well this gene matches the secondary preference of a power limit proportional to `energy_preference`.
    ///
    /// Prefer power limits proportional to (1 - energy_preference / 2)
    /// So maximum power at runtime-oriented, and half power at energy-oriented
    pub fn alignment(&self, energy_preference: f32) -> f32 {
        let target_power = 1.0 - 0.5 * energy_preference;
        1.0 - (self.powercap - target_power).abs().clamp(self.min_power, self.max_power)
    }
}

impl Gene for PowerGene {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let powercap = self.powercap * t + other.powercap * (1.0 - t);
        Self { powercap, min_power: self.min_power, max_power: self.max_power }
    }

    fn mutate(&mut self, strength: f32, immigration_similarity_threshold: f32) -> bool {
        let before = self.powercap;
        self.powercap += rand::random_range(-strength..=strength);
        self.powercap = self.powercap.max(self.min_power).min(self.max_power);

        (before - self.powercap).abs() <= immigration_similarity_threshold
    }

    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool {
        (self.powercap - other.powercap).abs() <= immigration_similarity_threshold
    }
}

impl std::ops::Deref for PowerGene {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.powercap
    }
}
