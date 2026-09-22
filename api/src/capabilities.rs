use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application.
///
/// These are provided by the application as JSON.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapabilities {
    /// The process ID of the application.
    pub pid: i32,
    /// Maximum number of threads the application may use.
    pub max_threads: u16,
}
