use crate::message::{Demand, Sample};

use super::Controller;

pub struct GeneticController {
    pub population: Vec<Chromosome>,
    /// Keeps track of the current index. The population is reset every
    /// `population_size` iterations. In between, we want every chromosome
    /// to be executed once.
    population_idx: usize,
    samples: Vec<Sample>,
    settings: GeneticControllerSettings,
}

pub struct GeneticControllerSettings {
    pub max_threads: i32,
    pub score_fn: fn(Sample) -> f32,
    pub population_size: usize,
    pub survival_rate: f32,
    pub mutation_rate: f32,
}

impl GeneticController {
    pub fn new(settings: GeneticControllerSettings) -> Self {
        let population = (0..settings.population_size)
            .map(|_| Chromosome::rand(settings.max_threads))
            .collect();

        Self {
            population,
            population_idx: 0,
            samples: Vec::new(),
            settings,
        }
    }

    fn evolve(&mut self) {
        let mut samples_new = Vec::new();
        std::mem::swap(&mut self.samples, &mut samples_new);

        self.population.iter_mut()
            .zip(samples_new.into_iter())
            .for_each(|(chromosome, sample)| {
                chromosome.score = (self.settings.score_fn)(sample);
            });

        self.population.sort_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap()
        });

        // Keep the N% best performing chromosomes
        let n = (self.settings.population_size as f32 * self.settings.survival_rate).floor() as usize;

        // Replace the remaining chromosomes by children of the best performing chromosomes
        for i in n..self.settings.population_size {
            let parent1 = &self.population[rand::random_range(0..n)];
            let parent2 = &self.population[rand::random_range(0..n)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.settings.mutation_rate {
                child.mutate(self.settings.max_threads);
            }

            self.population[i] = child;
        }

        // We want to sort the population by recommended thread-count
        // here, to minimise changes in the running program.
        self.population.sort_by(|a, b| {
            a.num_threads.partial_cmp(&b.num_threads).unwrap()
        });
    }
}

impl Controller for GeneticController {
    fn sample_received(&mut self, sample: Sample) {
        self.samples.push(sample);
        if self.samples.len() >= self.settings.population_size {
            self.evolve();
        }
    }

    fn next_demand(&mut self) -> Demand {
        self.population_idx = (self.population_idx + 1) % self.settings.population_size;
        let num_threads = self.population[self.population_idx].num_threads;
        Demand { num_threads }
    }
}

#[derive(Clone)]
pub struct Chromosome {
    pub num_threads: i32,
    pub score: f32,
}

impl Chromosome {
    fn rand(max_threads: i32) -> Self {
        Self {
            num_threads: rand::random_range(1..=max_threads),
            score: 0.0,
        }
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
            score: 0.0,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, max_threads: i32) {
        self.num_threads += rand::random_range(0..=1) * 2 - 1;
        self.num_threads = self.num_threads.max(1).min(max_threads)
    }
}
