use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application, including its process ID
/// and the minimum and maximum number of threads it may use.
///
/// Power limit is deliberately not included, as it is a system-wide
/// setting and not specific to the application.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct Capabilities {
    /// The process ID of the application.
    pub pid: i32,

    /// Minimum number of threads the application may use. [default: 1]
    #[serde(default = "one")]
    pub min_threads: u16,
    /// Maximum number of threads the application may use.
    pub max_threads: u16,
}

fn one() -> u16 { 1 }
