use super::{Gene, lerp};

#[derive(Clone, Debug)]
pub struct ThreadGene {
    num_threads: u16,
    max_threads: u16,
}

impl ThreadGene {
    pub fn new(max_threads: u16) -> Self {
        let max_threads = max_threads.max(1);
        Self { num_threads: max_threads, max_threads }
    }

    pub fn rand(mut self) -> Self {
        self.num_threads = rand::random_range(1..=self.max_threads);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.num_threads = lerp(1.0, self.max_threads as f32, t).round() as u16;
        self
    }

    pub fn alignment(&self, global_thread_count: u16) -> f32 {
        let available_cores = crate::HARDWARE.available_cores();
        let total_threads = self.num_threads + global_thread_count;
        if total_threads <= available_cores {
            1.0
        } else {
            let oversubscription = total_threads - available_cores;
            1.0 - (oversubscription as f32 / available_cores as f32).clamp(0.0, 1.0)
        }
    }
}

impl Gene for ThreadGene {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        let num_threads = (self.num_threads as f32 * t + other.num_threads as f32 * (1.0 - t)).round() as u16;
        Self { num_threads, max_threads: self.max_threads }
    }

    fn mutate(&mut self, strength: f32, _immigration_similarity_threshold: f32) -> bool {
        if rand::random_bool(strength as f64) {
            self.num_threads = if rand::random_bool(0.5) { self.num_threads.saturating_sub(1) } else { self.num_threads.saturating_add(1) };
            self.num_threads = self.num_threads.clamp(1, self.max_threads);
            return false;
        }
        return true;
    }

    fn is_similar_to(&self, other: &Self, _immigration_similarity_threshold: f32) -> bool {
        self.num_threads == other.num_threads
    }
}

impl PartialEq for ThreadGene {
    fn eq(&self, other: &Self) -> bool {
        self.num_threads == other.num_threads
    }
}

impl std::ops::Deref for ThreadGene {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.num_threads
    }
}
