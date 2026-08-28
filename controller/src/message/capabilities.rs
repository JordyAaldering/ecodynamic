use serde::{Deserialize, Serialize};

/// Contains the capabilities reported by the application through the socket.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapabilitiesResp {
    /// The process ID of the application.
    pub pid: i32,
    /// Maximum number of threads the application may use.
    pub max_threads: u16,
}

/// Represents the configuration of the resource controller, the capabilities
/// reported by the application, and the capabilities of the hardware.
#[derive(Clone, Debug)]
pub struct Capabilities {
    pub pid: i32,
    pub max_threads: u16,
    pub thread_control: bool,
    pub pinning_control: bool,
    pub power_control: bool,
    /// Minimum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    pub min_power: f32,
    /// Maximum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    pub max_power: f32,
}
