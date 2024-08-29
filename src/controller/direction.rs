#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn towards(from: f32, to: f32) -> Direction {
        if from < to {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl std::ops::Mul<f32> for Direction {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
