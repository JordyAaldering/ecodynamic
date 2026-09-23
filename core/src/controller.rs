use ecodynamic_api::{Demand, Sample};

pub trait Controller {
    fn get_demand(&self, index: usize) -> Demand;

    fn evolve(&mut self, samples: Vec<Sample>);
}
