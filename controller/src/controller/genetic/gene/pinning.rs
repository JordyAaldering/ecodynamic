use crate::PinningStrategy;

use super::Gene;

impl Gene for PinningStrategy {
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
