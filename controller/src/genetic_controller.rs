use crate::{Builder, Controller};

pub struct GeneticBuilder {
    pub population_size: usize,
}

pub struct GeneticController {
    max_threads: i32,
    population: Vec<Chromosome>,
}

impl Builder<GeneticController> for GeneticBuilder {
    fn build(&self, max_threads: i32) -> GeneticController {
        let population = (0..self.population_size).map(|_| Chromosome::rand(max_threads)).collect();
        GeneticController {
            max_threads,
            population,
        }
    }
}

impl Controller for GeneticController {
    fn adjust_threads(&mut self, samples: Vec<f32>) {
        todo!()
    }

    fn num_threads(&self) -> i32 {
        todo!()
    }
}

struct Chromosome {
    num_threads: i32,
}

impl Chromosome {
    fn rand(max_threads: i32) -> Self {
        Self {
            num_threads: rand::random_range(1..=max_threads),
        }
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
