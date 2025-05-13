use controller::Sample;
use non_dominated_sort::{non_dominated_sort, DominanceOrd};

#[derive(clap::ValueEnum)]
#[derive(Copy, Clone, Debug)]
pub enum ScoreFunction {
    Runtime,
    Energy,
    Pareto,
}

impl ScoreFunction {
    pub fn score(self, samples: Vec<Sample>) -> Vec<f32> {
        use ScoreFunction::*;
        match self {
            Runtime => {
                samples.into_iter().map(|x| x.runtime).collect()
            },
            Energy => {
                samples.into_iter().map(|x| x.energy).collect()
            },
            Pareto => {
                let num_samples = samples.len();
                let indexed: Vec<(usize, Sample)> = samples.into_iter().enumerate().collect();
                let mut fronts = non_dominated_sort(&indexed, &SampleDominanceOrd);
                println!("{:#?}", fronts);

                let mut scores = vec![0.0; num_samples];
                let mut dominated_count = 0.0;
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
    type T = (usize, Sample);

    fn dominates(&self, (_, a): &Self::T, (_, b): &Self::T) -> bool {
        a.runtime < b.runtime
            && a.energy < b.energy
    }
}
