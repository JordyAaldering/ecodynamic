use std::fmt;

use non_dominated_sort::{non_dominated_sort, DominanceOrd};

use crate::Sample;

#[derive(Copy, Clone, Debug)]
#[derive(clap::ValueEnum)]
pub enum ScoreFunction {
    Energy,
    Runtime,
    /// Energy-Delay Product.
    EDP,
    /// Energy-Squared-Delay Product.
    E2DP,
    /// Pareto-optimum.
    Pareto,
    /// Energy-runtime preference `energy^x * runtime^(1-x)`.
    Slider,
}

impl ScoreFunction {
    pub fn score(self, samples: Vec<Sample>, x: f32) -> Vec<f32> {
        use ScoreFunction::*;
        match self {
            Runtime => samples.into_iter().map(|s| s.runtime).collect(),
            Energy => samples.into_iter().map(|s| s.energy).collect(),
            EDP => samples.into_iter().map(|s| s.energy * s.runtime).collect(),
            E2DP => samples.into_iter().map(|s| s.energy * s.energy * s.runtime).collect(),
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
            Slider => samples.into_iter().map(|s|
                s.energy.powf(x) * s.runtime.powf(1.0 - x)).collect(),
        }
    }
}

impl fmt::Display for ScoreFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ScoreFunction::*;
        match self {
            Energy => write!(f, "energy"),
            Runtime => write!(f, "runtime"),
            EDP => write!(f, "edp"),
            E2DP => write!(f, "e2dp"),
            Pareto => write!(f, "pareto"),
            Slider => write!(f, "slider"),
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
