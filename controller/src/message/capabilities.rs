use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application.
///
/// Power limit is deliberately not included, as it is a system-wide setting.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    /// The process ID of the application.
    pub pid: i32,
    /// Maximum number of threads the application may use.
    pub max_threads: u16,
}
