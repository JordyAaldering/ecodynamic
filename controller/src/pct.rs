use std::ops;

use crate::dir::Direction;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Percentage(u8);

impl Percentage {
    pub fn new(pct: u8) -> Self {
        Self(pct)
    }

    pub fn adjust(&mut self, pct: Percentage, dir: Direction) {
        self.0 = match dir {
            Direction::Up => self.0.saturating_add(pct.0).min(100),
            Direction::Down => self.0.saturating_sub(pct.0),
        }
    }
}

impl ops::Deref for Percentage {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::AddAssign<Self> for Percentage {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_add(rhs.0).min(100);
    }
}

impl ops::DivAssign<u8> for Percentage {
    fn div_assign(&mut self, rhs: u8) {
        self.0 = (self.0 / rhs).max(1);
    }
}
