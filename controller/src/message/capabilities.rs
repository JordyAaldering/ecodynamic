use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application and the hardware.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Capabilities {
    /// The process ID of the application.
    pub pid: i32,
    /// Maximum number of threads the application may use.
    pub max_threads: u16,
    /// Minimum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[serde(skip)]
    pub power_min: f32,
    /// Maximum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[serde(skip)]
    pub power_max: f32,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            pid: 0,
            max_threads: 1,
            power_min: 0.1,
            power_max: 1.0,
        }
    }
}
