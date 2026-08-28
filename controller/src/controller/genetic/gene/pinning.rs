use super::Gene;

/// Strategy used to pin the threads of a chromosome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinningGene {
    /// Let the OS scheduler decide thread placement.
    Free,
}

impl PinningGene {
    pub fn new() -> Self {
        Self::Free
    }

    pub fn rand(self) -> Self {
        self
    }

    pub fn lerp(self, _t: f32) -> Self {
        self
    }
}

impl Gene for PinningGene {
    fn crossover(&self, _other: &Self, _t: f32) -> Self {
        *self
    }

    fn mutate(&mut self, _strength: f32, _immigration_similarity_threshold: f32) -> bool {
        true
    }

    fn is_similar_to(&self, other: &Self, _immigration_similarity_threshold: f32) -> bool {
        self == other
    }
}
