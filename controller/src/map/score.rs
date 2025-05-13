use clap::ValueEnum;

use crate::Sample;

#[derive(Copy, Clone, Debug)]
#[derive(ValueEnum)]
pub enum ScoreFunction {
    Energy,
    Runtime,
}

impl ScoreFunction {
    pub fn score(self, samples: Vec<Sample>) -> Vec<f32> {
        use ScoreFunction::*;
        match self {
            Runtime => samples.into_iter().map(|x| x.runtime).collect(),
            Energy => samples.into_iter().map(|x| x.energy).collect()
        }
    }
}
