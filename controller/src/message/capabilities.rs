use serde::{Deserialize, Serialize};

/// Represents the capabilities of an application.
///
/// Power limit is deliberately not included, as it is a system-wide setting.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct Capabilities {
    pub pid: i32,
    #[serde(default = "one")]
    pub min_threads: u16,
    pub max_threads: u16,
}

fn one() -> u16 { 1 }
