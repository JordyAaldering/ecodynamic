use serde::{Deserialize, Serialize};

/// System-wide demands that have to be set by this controller.
#[derive(Clone, Debug)]
pub struct GlobalDemand {
    /// Maximum allowed CPU power as a fraction beteen (0,1].
    pub powercap_pct: f32,
}

/// Application-specific demands that have to be set by the controlled application.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocalDemand {
    /// Recommended number of threads to use for the next parallel iteration.
    pub threads_pct: f32,
}
