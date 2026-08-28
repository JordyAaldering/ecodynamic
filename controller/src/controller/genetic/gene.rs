use crate::HARDWARE;

use super::chromosome::ChromosomeConfig;

pub trait Gene: Clone + std::fmt::Debug + PartialEq {
    fn crossover(&self, other: &Self, t: f32) -> Self;
    /// Randomly mutate the gene, returning whether little enough mutation has occurred that the `prev_score` of the chromosome may be reused.
    fn mutate(&mut self, config: &ChromosomeConfig, strength: f32, immigration_similarity_threshold: f32) -> bool;
    /// Whether the two genes are close enough that a `prev_score` may be shared/reused between them.
    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThreadGene(u16);

impl ThreadGene {
    pub fn random(config: &ChromosomeConfig) -> ThreadGene {
        let num_threads = rand::random_range(1..=config.max_threads);
        Self(num_threads)
    }

    pub fn spread(config: &ChromosomeConfig, t: f32) -> ThreadGene {
        let num_threads = lerp(1.0, config.max_threads as f32, t).round() as u16;
        Self(num_threads)
    }

    pub fn alignment(&self, global_thread_count: u16) -> f32 {
        let available_cores = HARDWARE.available_cores();
        let total_threads = self.0 + global_thread_count;
        if total_threads <= available_cores {
            1.0
        } else {
            let oversubscription = total_threads - available_cores;
            1.0 - (oversubscription as f32 / available_cores as f32).clamp(0.0, 1.0)
        }
    }
}

impl Gene for ThreadGene {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let num_threads = (self.0 as f32 * t + other.0 as f32 * (1.0 - t)).round() as u16;
        Self(num_threads)
    }

    fn mutate(&mut self, config: &ChromosomeConfig, strength: f32, _immigration_similarity_threshold: f32) -> bool {
        if rand::random_bool(strength as f64) {
            self.0 = if rand::random_bool(0.5) { self.0.saturating_sub(1) } else { self.0.saturating_add(1) };
            self.0 = self.0.clamp(1, config.max_threads);
            return false;
        }
        return true;
    }

    fn is_similar_to(&self, other: &Self, _immigration_similarity_threshold: f32) -> bool {
        self == other
    }
}

impl std::ops::Deref for ThreadGene {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Strategy used to pin the threads of a chromosome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinningGene {
    /// Let the OS scheduler decide thread placement.
    Free,
}

impl PinningGene {
    pub fn random(_config: &ChromosomeConfig) -> Self {
        PinningGene::Free
    }

    pub fn spread(_config: &ChromosomeConfig, _t: f32) -> Self {
        PinningGene::Free
    }

    pub fn alignment(&self, _config: &ChromosomeConfig) -> f32 {
        1.0
    }
}

impl Gene for PinningGene {
    fn crossover(&self, _other: &Self, _t: f32) -> Self {
        *self
    }

    fn mutate(&mut self, _config: &ChromosomeConfig, _strength: f32, _immigration_similarity_threshold: f32) -> bool {
        true
    }

    fn is_similar_to(&self, other: &Self, _immigration_similarity_threshold: f32) -> bool {
        self == other
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PowerGene(f32);

impl PowerGene {
    pub fn random(config: &ChromosomeConfig) -> Self {
        let powercap = rand::random_range(config.min_power..=config.max_power);
        Self(powercap)
    }

    pub fn spread(config: &ChromosomeConfig, t: f32) -> Self {
        let powercap = lerp(config.min_power, config.max_power, t);
        Self(powercap)
    }

    /// How well this gene matches the secondary preference of a power limit proportional to `energy_preference`.
    ///
    /// Prefer power limits proportional to (1 - energy_preference / 2)
    /// So maximum power at runtime-oriented, and half power at energy-oriented
    pub fn alignment(&self, energy_preference: f32) -> f32 {
        let target_power = 1.0 - 0.5 * energy_preference;
        1.0 - (self.0 - target_power).abs().clamp(0.0, 1.0)
    }
}

impl Gene for PowerGene {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let powercap = self.0 * t + other.0 * (1.0 - t);
        Self(powercap)
    }

    fn mutate(&mut self, config: &ChromosomeConfig, strength: f32, immigration_similarity_threshold: f32) -> bool {
        let before = self.0;
        self.0 += rand::random_range(-strength..=strength);
        self.0 = self.0.max(config.min_power).min(config.max_power);

        (before - self.0).abs() <= immigration_similarity_threshold
    }

    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool {
        (self.0 - other.0).abs() <= immigration_similarity_threshold
    }
}

impl std::ops::Deref for PowerGene {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
