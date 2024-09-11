pub struct Clamp<T> where T: Copy + std::cmp::PartialOrd {
    value: T,
    min: T,
    max: T,
}

impl<T> Clamp<T>
    where T: Copy + std::cmp::PartialOrd {

    pub fn new(value: T, min: T, max: T) -> Self {
        assert!(min <= value && value <= max);
        Clamp { value, min, max }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.clamp();
    }

    fn clamp(&mut self) {
        if self.value < self.min {
            self.value = self.min;
        } else if self.value > self.max {
            self.value = self.max;
        }
    }
}

impl<T> std::ops::AddAssign<T> for Clamp<T>
    where T: Copy + std::cmp::PartialOrd + std::ops::AddAssign {

    fn add_assign(&mut self, rhs: T) {
        self.value += rhs;
        self.clamp();
    }
}

impl<T> std::ops::SubAssign<T> for Clamp<T>
    where T: Copy + std::cmp::PartialOrd + std::ops::SubAssign {

    fn sub_assign(&mut self, rhs: T) {
        self.value -= rhs;
        self.clamp();
    }
}

impl<T> std::ops::MulAssign<T> for Clamp<T>
    where T: Copy + std::cmp::PartialOrd + std::ops::MulAssign {

    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;
        self.clamp();
    }
}

impl<T> std::ops::DivAssign<T> for Clamp<T>
    where T: Copy + std::cmp::PartialOrd + std::ops::DivAssign {

    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
        self.clamp();
    }
}

impl<T> std::fmt::Display for Clamp<T>
    where T: Copy + std::cmp::PartialOrd + std::fmt::Display {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}
