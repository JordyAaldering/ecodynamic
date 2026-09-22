use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application.
///
/// These are provided by the application as JSON.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapabilities {
    /// The process ID of the application.
    pid: i32,

    /// Maximum number of threads the application may use.
    pub max_threads: u16,
}

impl AppCapabilities {
    pub fn new(pid: i32, max_threads: u16) -> Self {
        assert!(max_threads > 0);
        Self { pid, max_threads }
    }
}
