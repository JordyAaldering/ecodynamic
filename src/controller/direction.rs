#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn _towards(from: i32, to: i32) -> Direction {
        if from <= to {
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

impl std::ops::Mul<f64> for Direction {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
