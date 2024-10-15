#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    #[inline]
    fn neg(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl std::ops::Mul<f64> for Direction {
    type Output = f64;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
