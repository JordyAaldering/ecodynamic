use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Demand {
    /// Maximum allowed CPU power as a fraction beteen (0,1].
    ///
    /// This is a system-wide demand that has to be set by the controller, not the controlled application.
    #[serde(skip)]
    pub powercap_pct: f32,

    /// Recommended number of threads to use for the next parallel iteration.
    pub num_threads: u16,
}
