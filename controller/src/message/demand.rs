use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Demand {
    /// Recommended number of threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    num_threads: Option<u16>,
    /// Recommemded system-wide CPU power limit.
    #[serde(skip)]
    powercap_pct: Option<f32>,
    /// Recommemded system-wide CPU energy--power preference.
    #[serde(skip)]
    cpufreq_epp: Option<u8>,
}

impl Demand {
    pub fn new() -> Self {
        Self { num_threads: None, powercap_pct: None, cpufreq_epp: None }
    }

    pub fn with_threads(mut self, num_threads: Option<u16>) -> Self {
        self.num_threads = num_threads;
        self
    }

    pub fn num_threads(&self, max_theads: u16) -> u16 {
        self.num_threads.unwrap_or(max_theads)
    }

    pub fn with_powercap(mut self, powercap: Option<f32>) -> Self {
        self.powercap_pct = powercap;
        self
    }

    pub fn powercap(&self, max_power_uw: u64) -> u64 {
        if let Some(powercap_pct) = self.powercap_pct {
            (powercap_pct * max_power_uw as f32).round() as u64
        } else {
            max_power_uw
        }
    }

    pub fn with_cpufreq_epp(mut self, cpufreq_epp: Option<u8>) -> Self {
        self.cpufreq_epp = cpufreq_epp;
        self
    }

    pub fn cpufreq_epp(&self) -> Option<u8> {
        self.cpufreq_epp
    }
}
