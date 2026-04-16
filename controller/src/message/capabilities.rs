use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Capabilities {
    /// Maximum number of threads that the application can use.
    /// If `None`, thread adjustment is disabled.
    pub max_threads: Option<u16>,
}
