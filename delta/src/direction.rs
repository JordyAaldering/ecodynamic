#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

impl From<bool> for Direction {
    fn from(value: bool) -> Self {
        if value {
            Self::Ascending
        } else {
            Self::Descending
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Direction {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

impl std::ops::Mul<i32> for Direction {
    type Output = i32;

    fn mul(self, rhs: i32) -> i32 {
        match self {
            Self::Ascending => rhs,
            Self::Descending => -rhs,
        }
    }
}

impl std::ops::Mul<f32> for Direction {
    type Output = f32;

    fn mul(self, rhs: f32) -> f32 {
        match self {
            Self::Ascending => rhs,
            Self::Descending => -rhs,
        }
    }
}
