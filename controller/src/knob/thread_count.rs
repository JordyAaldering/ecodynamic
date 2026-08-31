use super::lerp;

#[derive(Debug)]
pub struct ThreadCount {
    pub num_threads: u16,
    pub max_threads: u16,
    /// The number of threads that are already being used by other processes on the
    /// system at the time this chromosome was sampled. This is used to calculate the
    /// alignment of this chromosome with the system's current thread utilization.
    pub utilization: Option<u16>,
}

impl ThreadCount {
    pub fn new(max_threads: u16) -> Self {
        let max_threads = max_threads.max(1);
        Self {
            num_threads: max_threads,
            max_threads,
            utilization: None,
        }
    }

    pub fn rand(mut self) -> Self {
        self.num_threads = rand::random_range(1..=self.max_threads);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.num_threads = lerp(1.0, self.max_threads as f32, t).round() as u16;
        self
    }

    pub fn get_num_threads(&self) -> u16 {
        self.num_threads
    }

    pub fn set_utilization(&mut self, utilization: u16) {
        self.utilization = Some(utilization);
    }
}

impl PartialEq for ThreadCount {
    fn eq(&self, other: &Self) -> bool {
        self.num_threads == other.num_threads
    }
}

impl PartialOrd for ThreadCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num_threads.partial_cmp(&other.num_threads)
    }
}
