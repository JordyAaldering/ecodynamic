use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign, Div, DivAssign};

#[derive(Copy, Clone)]
pub struct Clamped<T: Copy + PartialOrd> {
    value: T,
    min: T,
    max: T,
}

impl<T: Copy + PartialOrd> Clamped<T> {
    pub fn new(value: T, min: T, max: T) -> Self {
        Clamped { value, min, max }
    }

    pub fn into(self) -> T {
        self.value
    }
}

impl<T: Copy + PartialOrd + Add<Output = T>> Add<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn add(self, rhs: T) -> Self::Output {
        Clamped {
            value: clamp(self.value + rhs, self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

impl<T: Copy + PartialOrd + Add<Output = T>> AddAssign<T> for Clamped<T> {
    fn add_assign(&mut self, rhs: T) {
        self.value = clamp(self.value + rhs, self.min, self.max);
    }
}

impl<T: Copy + PartialOrd + Div<Output = T>> Div<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn div(self, rhs: T) -> Self::Output {
        Clamped {
            value: clamp(self.value / rhs, self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

impl<T: Copy + PartialOrd + Div<Output = T>> DivAssign<T> for Clamped<T> {
    fn div_assign(&mut self, rhs: T) {
        self.value = clamp(self.value / rhs, self.min, self.max);
    }
}

fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
