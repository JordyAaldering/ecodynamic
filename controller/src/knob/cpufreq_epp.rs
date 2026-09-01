use super::lerp;

/// CPUFreq exposes the Energy Performance Preference (EPP) knob for CPU frequency scaling.
/// EPP is a value that can be set to influence the trade-off between performance and energy efficiency.
/// Lower values prioritize performance, while higher values prioritize energy efficiency.
///
/// This seems perfect for our usecase.
///
/// This knob serves as an alternative to the Powercap knob.
#[derive(Debug)]
pub struct CPUFreqEpp {
    pub epp: u8,
}

impl CPUFreqEpp {
    pub fn rand(min_epp: u8, max_epp: u8) -> Self {
        Self {
            epp: rand::random_range(min_epp..=max_epp),
        }
    }

    pub fn lerp(min_epp: u8, max_epp: u8, t: f32) -> Self {
        Self {
            epp: lerp(min_epp as f32, max_epp as f32, t) as u8,
        }
    }

    pub fn get_epp(&self) -> u8 {
        self.epp
    }
}

impl PartialEq for CPUFreqEpp {
    fn eq(&self, other: &Self) -> bool {
        self.epp == other.epp
    }
}

impl PartialOrd for CPUFreqEpp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.epp.partial_cmp(&other.epp)
    }
}
