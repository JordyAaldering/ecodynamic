use crate::ThreadCount;

use super::Gene;

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
