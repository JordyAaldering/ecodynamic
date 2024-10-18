pub struct ThreadCount {
    n: f32,
    max: f32,
}

impl ThreadCount {
    #[inline]
    pub fn new(max: usize) -> Self {
        Self { n: max as f32, max: max as f32 }
    }

    #[inline]
    fn clamp(&mut self) {
        self.n = f32::max(1.0, f32::min(self.n, self.max));
    }
}

impl std::ops::AddAssign<f32> for ThreadCount {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.n += rhs;
        self.clamp();
    }
}

impl std::ops::Deref for ThreadCount {
    type Target = f32;

    #[inline]
    fn deref(&self) -> &f32 {
        &self.n
    }
}
