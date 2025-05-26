use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction};

use super::Controller;

pub struct GeneticController {
    max_threads: i32,
    power_limit_uw: u64,
    population: Vec<Chromosome>,
    sample_index: usize,
    config: GeneticControllerConfig,
}

#[derive(Clone, Debug)]
#[derive(Parser)]
pub struct GeneticControllerConfig {
    #[arg(long, default_value_t = ScoreFunction::Pareto)]
    pub score: ScoreFunction,

    /// Genetic algorithm survival rate.
    #[arg(long, default_value_t = 0.50)]
    pub survival_rate: f32,

    /// Genetic algorithm mutation rate.
    #[arg(long, default_value_t = 0.25)]
    pub mutation_rate: f32,

    /// Genetic algorithm immigration rate.
    #[arg(long, default_value_t = 0.0)]
    pub immigration_rate: f32,
}

impl GeneticController {
    pub fn new(max_threads: i32, power_limit_uw: u64, population_size: usize, config: GeneticControllerConfig) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        // I.e. value = lower + i * (upper - lower) / length
        let population = (0..population_size).map(|i| {
                //let num_threads = 1 + (i as f64 * (max_threads - 1) as f64 / (population_size - 1) as f64).round() as i32;
                let num_threads = max_threads;
                let min_power_uw = power_limit_uw / 4;
                let power_limit_uw = min_power_uw + (i as u64 * (power_limit_uw - min_power_uw) / (population_size as u64 - 1));
                Chromosome::new(num_threads, power_limit_uw)
            }).collect();

        Self {
            max_threads,
            power_limit_uw,
            population,
            sample_index: 0,
            config,
        }
    }

    fn sort(&mut self, scores: Vec<f32>) {
        let mut permutation = permutation::sort_by(&scores, |a, b| a.partial_cmp(b).unwrap());
        permutation.apply_slice_in_place(&mut self.population);
    }
}

impl Controller for GeneticController {
    fn evolve(&mut self, samples: Vec<Sample>) {
        // Reset sample index for next
        self.sample_index = 0;

        self.sort(self.config.score.score(samples));

        let population_size = self.population.len();
        let survival_count = (population_size as f32 * self.config.survival_rate).round() as usize;
        let immigration_count = (population_size as f32 * self.config.immigration_rate).round() as usize;
        let immigration_start = population_size - immigration_count;

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.config.mutation_rate {
                child.mutate(self.max_threads, self.power_limit_uw);
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..population_size {
            self.population[i] = Chromosome::rand(self.max_threads, self.power_limit_uw);
        }

        // To minimise changes in the runtime we sort by the recommended thread-count
        self.population.sort_by_key(|c| c.power_limit_uw);
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let chromosome = &self.population[self.sample_index];
        self.sample_index += 1;

        let global = GlobalDemand { power_limit_uw: chromosome.power_limit_uw };
        let local = LocalDemand { num_threads: chromosome.num_threads };
        (global, local)
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    num_threads: i32,
    power_limit_uw: u64,
}

impl Chromosome {
    fn new(num_threads: i32, power_limit_uw: u64) -> Self {
        Self { num_threads, power_limit_uw }
    }

    /// Generate a random chromosome for immigration
    fn rand(max_threads: i32, max_power_uw: u64) -> Self {
        //let num_threads = rand::random_range(1..=max_threads);
        let num_threads = max_threads;
        let power_limit_uw = rand::random_range((max_power_uw / 2)..=max_power_uw);
        Self::new(num_threads, power_limit_uw)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
            power_limit_uw: (self.power_limit_uw + other.power_limit_uw) / 2,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, _max_threads: i32, max_power_uw: u64) {
        //self.num_threads += rand::random_range(0..=1) * 2 - 1;
        //self.num_threads = self.num_threads.max(1).min(max_threads);

        if rand::random_bool(0.5) {
            self.power_limit_uw += rand::random_range(0..=(max_power_uw / 10));
            self.power_limit_uw = self.power_limit_uw.min(max_power_uw);
        } else {
            self.power_limit_uw -= rand::random_range(0..=(max_power_uw / 10));
            self.power_limit_uw = self.power_limit_uw.max(max_power_uw / 2);
        }
    }
}
