use crate::{PinningStrategy, Powercap, ThreadCount};

pub trait Gene {
    fn crossover(&self, other: &Self, t: f32) -> Self;

    fn mutate(&mut self, strength: f32) -> f32;

    fn similarity(&self, other: &Self) -> f32;
}

impl<G: Gene> Gene for Option<G> {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        self.as_ref().map(|gene| gene.crossover(other.as_ref().unwrap(), t))
    }

    fn mutate(&mut self, strength: f32) -> f32 {
        self.as_mut().map(|gene| gene.mutate(strength)).unwrap_or(1.0)
    }

    fn similarity(&self, other: &Self) -> f32 {
        self.as_ref().map(|gene| gene.similarity(other.as_ref().unwrap())).unwrap_or(1.0)
    }
}

impl ThreadCount {
    pub fn alignment(&self, hw_available_threads: u16) -> f32 {
        let total_threads = self.num_threads + self.utilization.unwrap();
        if total_threads <= hw_available_threads {
            1.0
        } else {
            let oversubscription = total_threads - hw_available_threads;
            1.0 - (oversubscription as f32 / hw_available_threads as f32).clamp(0.0, 1.0)
        }
    }
}

impl Gene for ThreadCount {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let num_threads = (self.num_threads as f32 * t + other.num_threads as f32 * (1.0 - t)).round() as u16;
        Self { num_threads, max_threads: self.max_threads, utilization: None }
    }

    fn mutate(&mut self, strength: f32) -> f32 {
        if rand::random_bool(strength as f64) {
            self.num_threads = if rand::random_bool(0.5) { self.num_threads.saturating_sub(1) } else { self.num_threads.saturating_add(1) };
            self.num_threads = self.num_threads.clamp(1, self.max_threads);
            return 0.0;
        }
        return 1.0;
    }

    fn similarity(&self, other: &Self) -> f32 {
        (self.num_threads == other.num_threads).into()
    }
}

impl Gene for PinningStrategy {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        if t < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn mutate(&mut self, _strength: f32) -> f32 {
        1.0
    }

    fn similarity(&self, other: &Self) -> f32 {
        (self == other).into()
    }
}

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
