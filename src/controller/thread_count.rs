pub struct ThreadCount {
    n: f64,
    max: f64,
}

impl ThreadCount {
    #[inline]
    pub fn new(max: i32) -> Self {
        Self { n: max as f64, max: max as f64 }
    }

    #[inline]
    fn clamp(&mut self) {
        self.n = f64::max(1.0, f64::min(self.n, self.max));
    }
}

impl std::ops::AddAssign<f64> for ThreadCount {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.n += rhs;
        self.clamp();
    }
}

impl std::ops::Deref for ThreadCount {
    type Target = f64;

    #[inline]
    fn deref(&self) -> &f64 {
        &self.n
    }
}
