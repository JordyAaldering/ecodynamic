mod threads;
mod pinning;
mod power;

pub use threads::ThreadGene;
pub use pinning::PinningGene;
pub use power::PowerGene;

pub trait Gene {
    fn crossover(&self, other: &Self, t: f32) -> Self;

    fn mutate(&mut self, strength: f32, immigration_similarity_threshold: f32) -> bool;

    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool;
}

impl<G: Gene> Gene for Option<G> {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        self.as_ref().map(|gene| gene.crossover(other.as_ref().unwrap(), t))
    }

    fn mutate(&mut self, strength: f32, immigration_similarity_threshold: f32) -> bool {
        self.as_mut().map(|gene| gene.mutate(strength, immigration_similarity_threshold)).unwrap_or(true)
    }

    fn is_similar_to(&self, other: &Self, immigration_similarity_threshold: f32) -> bool {
        self.as_ref().map(|gene| gene.is_similar_to(other.as_ref().unwrap(), immigration_similarity_threshold)).unwrap_or(true)
    }
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
