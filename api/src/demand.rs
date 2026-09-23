use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Demand {
    /// Recommended number of threads.
    pub num_threads: u16,
    /// Recommemded system-wide CPU power limit.
    #[serde(skip)]
    pub powercap_pct: f32,
}
