#[derive(Copy, Clone)]
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

impl std::ops::Mul<i32> for Direction {
    type Output = i32;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}

impl std::ops::Mul<f32> for Direction {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
