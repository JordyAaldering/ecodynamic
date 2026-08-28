use crate::HARDWARE;

use super::GeneticConfig;

/// A single optional, independently evolvable trait of a [Chromosome].
///
/// Each gene knows how to initialize, breed and mutate itself, which keeps
/// `Chromosome`/`GeneticController` free of per-feature `if config.do_x_control` branches.
/// Genes that can be disabled represent that as a dedicated variant (e.g. `Fixed`/`Disabled`)
/// rather than as a flag, so callers never need to branch on whether a gene is active.
pub trait Gene: Clone + PartialEq + std::fmt::Debug {
    /// Deterministic, evenly spread sample across the valid range for `t` in `[0,1]`.
    fn spread(t: f32, config: &GeneticConfig, max_threads: u16) -> Self;
    /// Uniformly random sample, used for immigration when too few individuals are needed to spread evenly.
    fn random(config: &GeneticConfig, max_threads: u16) -> Self;
    fn crossover(&self, other: &Self, t: f32) -> Self;
    fn mutate(&mut self, config: &GeneticConfig, max_threads: u16);
    /// Whether the two genes are close enough that a `prev_score` may be shared/reused between them.
    fn is_similar_to(&self, other: &Self, config: &GeneticConfig) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThreadGene {
    /// Thread count is genetically tuned.
    Controlled(u16),
    /// Thread control is disabled: always use the maximum available thread count.
    Fixed(u16),
}

impl ThreadGene {
    pub fn value(&self, max_threads: u16) -> u16 {
        match self {
            ThreadGene::Controlled(n) => (*n).clamp(1, max_threads.max(1)),
            ThreadGene::Fixed(_) => max_threads.max(1),
        }
    }

    fn raw(&self) -> u16 {
        match self {
            ThreadGene::Controlled(n) | ThreadGene::Fixed(n) => *n,
        }
    }

    /// How well this gene matches the secondary preference of not oversubscribing available cores.
    pub fn alignment(&self, global_thread_count: u16) -> f32 {
        match self {
            ThreadGene::Fixed(_) => 1.0,
            ThreadGene::Controlled(n) => {
                let available_cores = HARDWARE.available_cores();
                let total_threads = n + global_thread_count;
                if total_threads <= available_cores {
                    1.0
                } else {
                    let oversubscription = total_threads - available_cores;
                    debug_assert!(oversubscription > 0);
                    1.0 - (oversubscription as f32 / available_cores as f32).clamp(0.0, 1.0)
                }
            }
        }
    }
}

impl Gene for ThreadGene {
    fn spread(t: f32, config: &GeneticConfig, max_threads: u16) -> Self {
        if config.do_thread_control {
            ThreadGene::Controlled(lerp(1.0, max_threads.max(1) as f32, t).round() as u16)
        } else {
            ThreadGene::Fixed(max_threads.max(1))
        }
    }

    fn random(config: &GeneticConfig, max_threads: u16) -> Self {
        if config.do_thread_control {
            ThreadGene::Controlled(rand::random_range(1..=max_threads.max(1)))
        } else {
            ThreadGene::Fixed(max_threads.max(1))
        }
    }

    fn crossover(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (ThreadGene::Controlled(a), ThreadGene::Controlled(b)) => {
                ThreadGene::Controlled((*a as f32 * t + *b as f32 * (1.0 - t)).round() as u16)
            }
            _ => ThreadGene::Fixed(self.raw().max(other.raw())),
        }
    }

    fn mutate(&mut self, config: &GeneticConfig, max_threads: u16) {
        if let ThreadGene::Controlled(n) = self {
            if rand::random_bool(config.mutation_strength as f64) {
                *n = if rand::random_bool(0.5) { n.saturating_sub(1) } else { n.saturating_add(1) };
                *n = (*n).clamp(1, max_threads);
            }
        }
    }

    fn is_similar_to(&self, other: &Self, _config: &GeneticConfig) -> bool {
        self == other
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PowerGene {
    /// Power limit is genetically tuned.
    Controlled(f32),
    /// Power control is disabled: always use the maximum power limit.
    Fixed,
}

impl PowerGene {
    pub fn value(&self) -> f32 {
        match self {
            PowerGene::Controlled(p) => *p,
            PowerGene::Fixed => 1.0,
        }
    }

    /// How well this gene matches the secondary preference of a power limit proportional to `energy_preference`.
    pub fn alignment(&self, energy_preference: f32) -> f32 {
        match self {
            PowerGene::Fixed => 1.0,
            PowerGene::Controlled(p) => {
                // Prefer power limits proportional to (1 - energy_preference / 2)
                // So maximum power at runtime-oriented, and half power at energy-oriented
                let target_power = 1.0 - 0.5 * energy_preference;
                1.0 - (p - target_power).abs().clamp(0.0, 1.0)
            }
        }
    }
}

impl Gene for PowerGene {
    fn spread(t: f32, config: &GeneticConfig, _max_threads: u16) -> Self {
        if config.do_power_control {
            PowerGene::Controlled(lerp(config.power_min, config.power_max, t))
        } else {
            PowerGene::Fixed
        }
    }

    fn random(config: &GeneticConfig, _max_threads: u16) -> Self {
        if config.do_power_control {
            PowerGene::Controlled(rand::random_range(config.power_min..=config.power_max))
        } else {
            PowerGene::Fixed
        }
    }

    fn crossover(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (PowerGene::Controlled(a), PowerGene::Controlled(b)) => PowerGene::Controlled(a * t + b * (1.0 - t)),
            _ => PowerGene::Fixed,
        }
    }

    fn mutate(&mut self, config: &GeneticConfig, _max_threads: u16) {
        if let PowerGene::Controlled(p) = self {
            *p += rand::random_range(-config.mutation_strength..=config.mutation_strength);
            *p = p.max(config.power_min).min(config.power_max);
        }
    }

    fn is_similar_to(&self, other: &Self, config: &GeneticConfig) -> bool {
        (self.value() - other.value()).abs() <= config.immigration_similarity_threshold
    }
}

/// Strategy used to pin/place the threads of a chromosome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinningStrategy {
    /// Let the OS scheduler decide thread placement.
    Free,
}

impl Gene for PinningStrategy {
    fn spread(_t: f32, _config: &GeneticConfig, _max_threads: u16) -> Self {
        PinningStrategy::Free
    }

    fn random(_config: &GeneticConfig, _max_threads: u16) -> Self {
        PinningStrategy::Free
    }

    fn crossover(&self, _other: &Self, _t: f32) -> Self {
        *self
    }

    fn mutate(&mut self, _config: &GeneticConfig, _max_threads: u16) {}

    fn is_similar_to(&self, other: &Self, _config: &GeneticConfig) -> bool {
        self == other
    }
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
