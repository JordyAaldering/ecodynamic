use crate::message::Demand;

use super::Controller;

pub struct GeneticController {
    max_threads: i32,
    samples: Vec<f32>,
    pub population: Vec<Chromosome>,
    settings: GeneticControllerSettings,
}

pub struct GeneticControllerSettings {
    pub population_size: usize,
    pub survival_rate: f32,
    pub mutation_rate: f32,
    pub immigration_rate: f32,
}

impl GeneticControllerSettings {
    fn survival_count(&self) -> usize {
        (self.population_size as f32 * self.survival_rate).round() as usize
    }

    fn immigration_count(&self) -> usize {
        (self.population_size as f32 * self.immigration_rate).round() as usize
    }
}

impl GeneticController {
    pub fn new(max_threads: i32, settings: GeneticControllerSettings) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        let population = (0..settings.population_size).map(|i| {
                let num_threads = 1 + (i as f64 * (max_threads - 1) as f64 / (settings.population_size - 1) as f64).round() as i32;
                Chromosome::new(num_threads)
            }).collect();

        Self {
            max_threads,
            samples: Vec::with_capacity(settings.population_size),
            population,
            settings,
        }
    }

    fn sort(&mut self, scores: Vec<f32>) {
        let mut permutation = permutation::sort_by(&scores, |a, b| a.partial_cmp(b).unwrap());
        permutation.apply_slice_in_place(&mut self.population);
    }
}

impl Controller for GeneticController {
    fn evolve(&mut self, scores: Vec<f32>) {
        self.sort(scores);

        let survival_count = self.settings.survival_count();
        let immigration_count = self.settings.immigration_count();
        let immigration_start = self.settings.population_size - immigration_count;

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.settings.mutation_rate {
                child.mutate(self.max_threads);
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..self.settings.population_size {
            self.population[i] = Chromosome::rand(self.max_threads);
        }

        // We want to sort the population by recommended thread-count
        // here, to minimise changes in the running program.
        self.population.sort();
    }

    fn get_demand(&self) -> Demand {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let idx = self.samples.len();
        let num_threads = self.population[idx].num_threads;
        Demand { num_threads }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chromosome {
    pub num_threads: i32,
}

impl Chromosome {
    fn new(num_threads: i32) -> Self {
        Self { num_threads }
    }

    /// Generate a random chromosome for immigration
    fn rand(max_threads: i32) -> Self {
        let num_threads = rand::random_range(1..=max_threads);
        Self::new(num_threads)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, max_threads: i32) {
        self.num_threads += rand::random_range(0..=1) * 2 - 1;
        self.num_threads = self.num_threads.max(1).min(max_threads)
    }
}
