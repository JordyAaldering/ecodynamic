use std::ops::{Mul, Neg};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}

impl Neg for Direction {
    type Output = Direction;

    #[inline]
    fn neg(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl Mul<i32> for Direction {
    type Output = i32;

    #[inline]
    fn mul(self, rhs: i32) -> i32 {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}

impl Mul<f32> for Direction {
    type Output = f32;

    #[inline]
    fn mul(self, rhs: f32) -> f32 {
        match self {
            Direction::Up => rhs,
            Direction::Down => -rhs,
        }
    }
}
