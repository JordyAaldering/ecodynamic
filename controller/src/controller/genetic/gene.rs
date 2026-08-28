mod threads;
mod pinning;
mod power;

pub use threads::ThreadGene;
pub use pinning::PinningGene;
pub use power::PowerGene;

pub trait Gene {
    fn crossover(&self, other: &Self, t: f32) -> Self;

    fn mutate(&mut self, strength: f32) -> f32;

    fn similarity(&self, other: &Self) -> f32;
}

impl<G: Gene> Gene for Option<G> {
    fn crossover(&self, other: &Self, t: f32) -> Self {
        self.as_ref().map(|gene| gene.crossover(other.as_ref().unwrap(), t))
    }

    fn mutate(&mut self, strength: f32) -> f32 {
        self.as_mut().map(|gene| gene.mutate(strength)).unwrap_or(1.0)
    }

    fn similarity(&self, other: &Self) -> f32 {
        self.as_ref().map(|gene| gene.similarity(other.as_ref().unwrap())).unwrap_or(1.0)
    }
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
	min + (max - min) * t
}
