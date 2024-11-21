use crate::{controller::*, letterbox::Letterbox, sample::Sample, selection::{FrequencyDist, Median, SelectionAlgorithm}};

pub struct Mtd {
    pub sample: Sample,
    pub sample_select: fn((f32, f32)) -> f32,
    letterbox: Letterbox,
    selection: Box<dyn SelectionAlgorithm>,
    controller: Box<dyn Controller>,
    pub num_threads: f32,
}

impl Mtd {
    pub fn energy_controller(max_threads: usize, samples_per_update: usize) -> Self {
        Self {
            sample: Sample::new(),
            sample_select: |(_runtime, energy)| energy,
            letterbox: Letterbox::new(samples_per_update),
            selection: Box::new(Median { }),
            controller: Box::new(EnergyController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn runtime_controller(max_threads: usize) -> Self {
        Self {
            sample: Sample::new(),
            sample_select: |(runtime, _energy)| runtime,
            letterbox: Letterbox::new(20),
            selection: Box::new(FrequencyDist::new(5, false)),
            controller: Box::new(RuntimeController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn fixed_controller(max_threads: usize) -> Self {
        Self {
            sample: Sample::new(),
            sample_select: |_| 0.0,
            letterbox: Letterbox::new(1),
            selection: Box::new(FrequencyDist::new(1, false)),
            controller: Box::new(FixedController::new(max_threads)),
            num_threads: max_threads as f32,
        }
    }

    pub fn install<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R + Send,
        R: Send,
    {
        self.sample.start();

        let res = f();

        let sample = self.sample.stop();
        let sample = (self.sample_select)(sample);
        self.update(sample);

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
