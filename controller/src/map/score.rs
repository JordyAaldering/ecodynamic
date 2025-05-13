use non_dominated_sort::{non_dominated_sort, DominanceOrd};

use crate::Sample;

#[derive(Copy, Clone, Debug)]
#[derive(clap::ValueEnum)]
pub enum ScoreFunction {
    Energy,
    Runtime,
    Pareto,
}

impl ScoreFunction {
    pub fn score(self, samples: Vec<Sample>) -> Vec<f32> {
        use ScoreFunction::*;
        match self {
            Runtime => samples.into_iter().map(|x| x.runtime).collect(),
            Energy => samples.into_iter().map(|x| x.energy).collect(),
            Pareto => {
                let mut fronts = non_dominated_sort(&samples, &SampleDominanceOrd);

                let mut scores = vec![-1.0; samples.len()];
                let mut dominated_count = 1.0;
                while !fronts.is_empty() {
                    for &index in fronts.current_front_indices() {
                        scores[index] = dominated_count;
                    }
                    dominated_count += 1.0;
                    fronts = fronts.next_front();
                }

                scores
            }
        }
    }
}

pub struct SampleDominanceOrd;

impl DominanceOrd for SampleDominanceOrd {
    type T = Sample;

    fn dominates(&self, a: &Self::T, b: &Self::T) -> bool {
        a.runtime < b.runtime
            && a.energy < b.energy
    }
}
