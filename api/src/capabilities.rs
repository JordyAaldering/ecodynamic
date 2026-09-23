use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application.
///
/// These are provided by the application as JSON upon establishing a connection with the server.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapabilities {
    /// The process ID of the application.
    pub pid: i32,
    /// Maximum number of threads the application may use.
    ///
    /// Note that this is different from the number of threads available on the system.
    /// An application may be limited to using only a subset of the available threads.
    pub max_threads: u16,
}
