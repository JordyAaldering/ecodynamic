pub struct Letterbox {
    pub runtimes: Vec<u64>,
    pub num_threads: i32,
}

impl Letterbox {
    pub fn new(max_threads: i32) -> Self {
        Letterbox {
            runtimes: Vec::new(),
            num_threads: max_threads,
        }
    }
}
