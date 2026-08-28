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
    fn crossover(&self, other: &Self, t: f32) -> Self {
        if t < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn mutate(&mut self, _strength: f32) -> f32 {
        1.0
    }

    fn similarity(&self, other: &Self) -> f32 {
        (self == other).into()
    }
}
