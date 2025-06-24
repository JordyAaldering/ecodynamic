use clap::Parser;

use crate::{GlobalDemand, LocalDemand, Sample, ScoreFunction};

use super::Controller;

const _THREADS_PCT_MIN: f32 = 0.1;
const POWER_PCT_MIN: f32 = 0.1;

pub struct GeneticController {
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
    pub fn new(population_size: usize, config: GeneticControllerConfig) -> Self {
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        // I.e. value = lower + i * (upper - lower) / length
        let population = (0..population_size).map(|i| {
                //let num_threads = 1 + (i as f64 * (max_threads - 1) as f64 / (population_size - 1) as f64).round() as i32;
                let threads_pct = 1.0;
                let power_limit_pct = POWER_PCT_MIN + (i as f32 * (1.0 - POWER_PCT_MIN) / (population_size - 1) as f32);
                Chromosome::new(threads_pct, power_limit_pct)
            }).collect();

        Self {
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
                child.mutate();
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..population_size {
            self.population[i] = Chromosome::rand();
        }

        // To minimise changes in the runtime we sort by the recommended power limit
        self.population.sort_by(|a, b| a.power_limit_pct.partial_cmp(&b.power_limit_pct).unwrap());
    }

    fn next_demand(&mut self) -> (GlobalDemand, LocalDemand) {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let chromosome = &self.population[self.sample_index];
        self.sample_index += 1;

        let global = GlobalDemand { power_limit_pct: chromosome.power_limit_pct };
        let local = LocalDemand { threads_pct: chromosome.threads_pct };
        (global, local)
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    threads_pct: f32,
    power_limit_pct: f32,
}

impl Chromosome {
    fn new(threads_pct: f32, power_limit_pct: f32) -> Self {
        Self { threads_pct, power_limit_pct }
    }

    /// Generate a random chromosome for immigration
    fn rand() -> Self {
        //let num_threads = rand::random_range(THREADS_PCT_MIN..=1.0);
        let num_threads = 1.0;
        let power_limit_pct = rand::random_range(POWER_PCT_MIN..=1.0);
        Self::new(num_threads, power_limit_pct)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            threads_pct: (self.threads_pct + other.threads_pct) / 2.0,
            power_limit_pct: (self.power_limit_pct + other.power_limit_pct) / 2.0,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self) {
        //self.num_threads += rand::random_range(-0.5..=0.5);
        //self.num_threads = self.num_threads.max(THREADS_PCT_MIN).min(1.0);

        self.power_limit_pct += rand::random_range(-0.5..=0.5);
        self.power_limit_pct = self.power_limit_pct.max(POWER_PCT_MIN).min(1.0);
    }
}
