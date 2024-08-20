#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Direction {
    Up = 1,
    Down = -1
}

impl Direction {
    pub fn towards(from: i32, to: i32) -> Direction {
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

impl std::ops::Mul<i32> for Direction {
    type Output = i32;

    fn mul(self, rhs: i32) -> Self::Output {
        self as i32 * rhs
    }
}
