use super::lerp;

#[derive(Debug)]
pub struct Powercap {
    pub powercap: f32,
    pub min_power: f32,
    pub max_power: f32,
}

impl Powercap {
    pub fn new(min_power: f32, max_power: f32) -> Self {
        Self { powercap: max_power, min_power, max_power }
    }

    pub fn rand(mut self) -> Self {
        self.powercap = rand::random_range(self.min_power..=self.max_power);
        self
    }

    pub fn lerp(mut self, t: f32) -> Self {
        self.powercap = lerp(self.min_power, self.max_power, t);
        self
    }

    pub fn get_powercap(&self) -> f32 {
        self.powercap
    }
}

impl PartialEq for Powercap {
    fn eq(&self, other: &Self) -> bool {
        self.powercap == other.powercap
    }
}

impl PartialOrd for Powercap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.powercap.partial_cmp(&other.powercap)
    }
}
