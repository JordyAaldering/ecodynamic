#[derive(Clone, Copy)]
pub enum Direction {
    Increasing,
    Decreasing,
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            Increasing => Decreasing,
            Decreasing => Increasing,
        }
    }
}

impl Into<f32> for Direction {
    fn into(self) -> f32 {
        use Direction::*;
        match self {
            Increasing => 1.0,
            Decreasing => -1.0,
        }
    }
}
