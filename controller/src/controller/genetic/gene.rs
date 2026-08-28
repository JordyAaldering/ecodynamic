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

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
