use crate::controller::direction::Direction;

pub struct ThreadCount {
    n: f64,
    max: f64,
}

impl ThreadCount {
    pub fn new(max: i32) -> Self {
        Self { n: max as f64, max: max as f64 }
    }

    pub fn adjust(&mut self, step_direction: Direction, step_size: f64) -> bool {
        let prev = self.n;
        self.n += step_direction * step_size;
        self.n = f64::max(1.0, f64::min(self.n, self.max));
        let changed = prev.round() != self.n.round();
        changed
    }
}

impl std::ops::Deref for ThreadCount {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.n
    }
}

//impl<T> std::fmt::Display for ThreadCount<T>
//    where T: Copy + std::cmp::PartialOrd + std::fmt::Display {
//
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.write_fmt(format_args!("{}", self.value))
//    }
//}
