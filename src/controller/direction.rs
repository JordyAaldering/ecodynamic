#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Direction {
    Up = 1,
    Down = -1
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
