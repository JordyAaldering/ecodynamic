use std::mem;

use crate::message::Demand;

use super::Controller;

pub struct GeneticController {
    samples: Vec<f32>,
    pub population: Vec<Chromosome>,
    settings: GeneticControllerSettings,
}

pub struct GeneticControllerSettings {
    pub max_threads: i32,
    pub population_size: usize,
    pub survival_rate: f32,
    pub mutation_rate: f32,
}

impl GeneticController {
    pub fn new(settings: GeneticControllerSettings) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        let population = (1..=settings.population_size).map(|i| {
                let num_threads = (i as f64 * settings.max_threads as f64 / (settings.population_size - 1) as f64).round() as i32;
                Chromosome::new(num_threads)
            }).collect();

        Self {
            samples: Vec::with_capacity(settings.population_size),
            population,
            settings,
        }
    }

    fn evolve(&mut self, scores: Vec<f32>) {
        self.population.iter_mut()
            .zip(scores.into_iter())
            .collect::<Vec<_>>()
            .sort_by(|(_, a), (_, b)|
                a.partial_cmp(&b).unwrap()
            );

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
    fn sample_received(&mut self, score: f32) {
        self.samples.push(score);
        if self.samples.len() >= self.settings.population_size {
            let mut samples_new = Vec::with_capacity(self.settings.population_size);
            mem::swap(&mut self.samples, &mut samples_new);
            self.evolve(samples_new);
        }
    }

    fn next_demand(&mut self) -> Demand {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let idx = self.samples.len();
        let num_threads = self.population[idx].num_threads;
        Demand { num_threads }
    }
}

#[derive(Clone)]
pub struct Chromosome {
    pub num_threads: i32,
    //pub score: f32,
}

impl Chromosome {
    fn new(num_threads: i32) -> Self {
        Self { num_threads }
    }

    // We will need this if we want to implement immigration
    //fn rand(max_threads: i32) -> Self {
    //    let num_threads = rand::random_range(1..=max_threads);
    //    Self::new(num_threads)
    //}

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
