use std::ops;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
}

impl ops::Neg for Direction {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl<Rhs: ops::Neg<Output = Rhs>> ops::Mul<Rhs> for Direction {
    type Output = Rhs;

    #[inline]
    fn mul(self, rhs: Rhs) -> Self::Output {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
