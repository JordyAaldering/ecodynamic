use super::lerp;

#[derive(Debug)]
pub struct Powercap {
    pub powercap: f32,
    pub min_power: f32,
    pub max_power: f32,
}

impl Powercap {
    pub fn rand(min_power: f32, max_power: f32) -> Self {
        Self {
            powercap: rand::random_range(min_power..=max_power),
            min_power,
            max_power,
        }
    }

    pub fn lerp(min_power: f32, max_power: f32, t: f32) -> Self {
        Self {
            powercap: lerp(min_power, max_power, t),
            min_power,
            max_power,
        }
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
