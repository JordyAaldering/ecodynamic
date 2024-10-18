use crate::{controller::{Controller, FrequencyDist, SelectionAlgorithm}, controller_energy::EnergyController, controller_runtime::RuntimeController, letterbox::Letterbox};

pub struct Mtd {
    letterbox: Letterbox,
    selection: FrequencyDist,
    controller: Box<dyn Controller>,
    pub num_threads: f32,
}

impl Mtd {
    pub fn energy_controller(max_threads: usize) -> Self {
        Self {
            letterbox: Letterbox::new(10),
            selection: FrequencyDist::new(4, true),
            controller: Box::new(EnergyController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn runtime_controller(max_threads: usize) -> Self {
        Self {
            letterbox: Letterbox::new(20),
            selection: FrequencyDist::new(5, false),
            controller: Box::new(RuntimeController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn install<F, R>(&mut self, pin_threads: bool, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        let pool = threadpool(self.num_threads() as usize, pin_threads);

        let mut rapl = rapl_energy::Rapl::now().unwrap();

        let res = pool.install(f);

        let energy = rapl.elapsed_mut().values().sum();
        self.update(energy);

        res
    }

    pub fn update(&mut self, sample: f32) {
        if let Some(samples) = self.letterbox.push(sample) {
            let samples = samples.into_iter().map(f64::from).collect();
            let tn = self.selection.find_best(samples) as f32;
            self.num_threads = self.controller.adjust_threads(tn);
        }
    }

    pub fn num_threads(&self) -> i32 {
        self.num_threads.round() as i32
    }
}

pub fn threadpool(num_threads: usize, pin_threads: bool) -> rayon::ThreadPool {
    let mut builder = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads);

    if pin_threads {
        let cores = core_affinity::get_core_ids().unwrap();
        let max_threads = cores.len();
        assert!(num_threads <= max_threads);
        let thread_indices: Vec<usize> = (0..max_threads).step_by(2)
            .chain((1..max_threads).step_by(2)).collect();

        builder = builder.start_handler(move |idx| {
            let thread_idx = thread_indices[idx];
            let core_id = cores[thread_idx];
            assert!(core_affinity::set_for_current(core_id));
        });
    }

    builder.build().unwrap()
}
