use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    /// Minimum number of threads that the application can use.
    /// If `None`, defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_threads: Option<u16>,
    /// Maximum number of threads that the application can use.
    /// If `None`, thread adjustment is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_threads: Option<u16>,

    /// Minimum power limit that the system may be configured to.
    /// If `None`, defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_powercap: Option<u16>,
    /// Maximum power limit that the system may be configured to.
    /// If `None`, defaults to the maximum power limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_powercap: Option<u16>,
}
