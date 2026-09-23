use ecodynamic_api::AppCapabilities;

#[derive(Clone, Copy)]
pub struct Capabilities<'a> {
    pub app: &'a AppCapabilities,
    pub ctx: &'a ServerCapabilities,
    pub hw: &'a HardwareCapabilities,
}

/// Represents the configuration of the resource controller and the capabilities of the hardware.
///
/// These are configured by the user as command line arguments.
#[derive(Clone, Debug, clap::Parser)]
pub struct ServerCapabilities {
    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while
    /// a value of 0 means that only runtime performance is optimised for.
    #[clap(short('a'), long, default_value_t = 0.9)]
    pub energy_preference: f32,
    /// Enable thread count control.
    #[clap(long)]
    pub thread_control: bool,
    /// Enable thread placement control.
    #[clap(long)]
    pub pinning_control: bool,
    /// Enable power limiting control.
    ///
    /// Should not be used in combination with [ServerCapabilities::cpufreq_epp_control].
    #[clap(long)]
    pub power_control: bool,
    /// Minimum allowed fraction of the maximum power limit.
    #[clap(long, default_value_t = 0.1)]
    pub min_power: f32,
    /// Maximum allowed fraction of the maximum power limit.
    #[clap(long, default_value_t = 1.0)]
    pub max_power: f32,
}

/// Represents the capabilities of the hardware.
///
/// These are derived on startup by the server.
///
/// TODO: a simpler approach would be to just require some
/// environment variables or configuration file to exist.
/// (Perhaps similarly for min_power and max_power, above)
#[derive(Clone, Debug)]
pub struct HardwareCapabilities {
    /// The number of threads available on the system.
    pub available_threads: u16,
    /// The maximum power draw of the processor in microwatts.
    pub max_power_uw: u64,
}
