use crate::State;

use super::{Gene, lerp};

#[derive(Debug)]
pub struct ThreadGene {
    num_threads: u16,
    max_threads: u16,
    /// The number of threads that are already being used by other processes on the
    /// system at the time this chromosome was sampled. This is used to calculate the
    /// alignment of this chromosome with the system's current thread utilization.
    utilization: Option<u16>,
}

impl ThreadGene {
    pub fn new(max_threads: u16) -> Self {
        let max_threads = max_threads.max(1);
        Self { num_threads: max_threads, max_threads, utilization: None }
    }

    pub fn rand(mut self) -> Self {
        self.num_threads = rand::random_range(1..=self.max_threads);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.num_threads = lerp(1.0, self.max_threads as f32, t).round() as u16;
        self
    }

    pub fn get_num_threads(&self) -> u16 {
        self.num_threads
    }

    pub fn store_state(&mut self, state: State) {
        self.utilization = Some(state.thread_utilization);
    }

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

impl Gene for ThreadGene {
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

impl PartialEq for ThreadGene {
    fn eq(&self, other: &Self) -> bool {
        self.num_threads == other.num_threads
    }
}

impl PartialOrd for ThreadGene {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num_threads.partial_cmp(&other.num_threads)
    }
}
