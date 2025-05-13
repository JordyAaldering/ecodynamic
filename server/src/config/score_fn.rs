use controller::Sample;

#[derive(clap::ValueEnum)]
#[derive(Copy, Clone, Debug)]
pub enum ScoreFunction {
    Runtime,
    Energy,
}

impl ScoreFunction {
    pub fn score(self, sample: Vec<Sample>) -> Vec<f32> {
        use ScoreFunction::*;
        match self {
            Runtime => sample.into_iter().map(|x| x.runtime).collect(),
            Energy => sample.into_iter().map(|x| x.energy).collect(),
        }
    }
}
