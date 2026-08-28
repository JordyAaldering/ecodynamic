use crate::HARDWARE;

use super::chromosome::ChromosomeConfig;

pub trait Gene: Clone + std::fmt::Debug + PartialEq {
    fn crossover(&self, other: &Self, t: f32) -> Self;
    /// Randomly mutate the gene, returning whether little enough mutation has occurred that the `prev_score` of the chromosome may be reused.
    fn mutate(&mut self, strength: f32, immigration_similarity_threshold: f32) -> bool;
    /// Whether the two genes are close enough that a `prev_score` may be shared/reused between them.
    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThreadGene {
    num_threads: u16,
    max_threads: u16,
}

impl ThreadGene {
    pub fn new(max_threads: u16) -> Self {
        let max_threads = max_threads.max(1);
        Self { num_threads: max_threads, max_threads }
    }

    pub fn rand(mut self) -> Self {
        self.num_threads = rand::random_range(1..=self.max_threads);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.num_threads = lerp(1.0, self.max_threads as f32, t).round() as u16;
        self
    }

    pub fn alignment(&self, global_thread_count: u16) -> f32 {
        let available_cores = HARDWARE.available_cores();
        let total_threads = self.num_threads + global_thread_count;
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
        let num_threads = (self.num_threads as f32 * t + other.num_threads as f32 * (1.0 - t)).round() as u16;
        Self { num_threads, max_threads: self.max_threads }
    }

    fn mutate(&mut self, strength: f32, _immigration_similarity_threshold: f32) -> bool {
        if rand::random_bool(strength as f64) {
            self.num_threads = if rand::random_bool(0.5) { self.num_threads.saturating_sub(1) } else { self.num_threads.saturating_add(1) };
            self.num_threads = self.num_threads.clamp(1, self.max_threads);
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
        &self.num_threads
    }
}

/// Strategy used to pin the threads of a chromosome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinningGene {
    /// Let the OS scheduler decide thread placement.
    Free,
}

impl PinningGene {
    pub fn new() -> Self {
        Self::Free
    }

    pub fn rand(self) -> Self {
        self
    }

    pub fn lerp(self, t: f32) -> Self {
        self
    }
}

impl Gene for PinningGene {
    fn crossover(&self, _other: &Self, _t: f32) -> Self {
        *self
    }

    fn mutate(&mut self, _strength: f32, _immigration_similarity_threshold: f32) -> bool {
        true
    }

    fn is_similar_to(&self, other: &Self, _immigration_similarity_threshold: f32) -> bool {
        self == other
    }
}

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

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
