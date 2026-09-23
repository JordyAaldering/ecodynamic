use clap::Parser;

/// Represents the capabilities of the hardware.
///
/// These are derived on startup by the server.
#[derive(Clone, Debug, Parser)]
pub struct HardwareCapabilities {
    /// The number of physical (performance) cores available on the system.
    #[arg(env = "CORE_COUNT")]
    pub core_count: u16,
    /// The number of logical threads available on the system.
    #[arg(env = "THREAD_COUNT")]
    pub thread_count: u16,

    /// The maximum power draw of the processor in microwatts.
    ///
    /// For example, this could be the value stored in `/sys/class/powercap/intel-rapl:0/constraint_0_max_power_uw`.
    #[arg(env = "MAX_POWER_UW")]
    pub max_power_uw: u64,
    /// The minimum allowed fraction of the maximum power limit.
    #[arg(env = "MIN_POWER_FRAC", default_value_t = 0.1)]
    pub min_power_frac: f32,
    /// The maximum allowed fraction of the maximum power limit.
    #[arg(env = "MAX_POWER_FRAC", default_value_t = 1.0)]
    pub max_power_frac: f32,

    /// Enable thread count control.
    #[clap(long, env = "DO_THREAD_CONTROL")]
    pub do_thread_control: bool,
    /// Enable thread placement control.
    #[clap(long, env = "DO_PINNING_CONTROL")]
    pub do_pinning_control: bool,
    /// Enable power limiting control.
    #[clap(long, env = "DO_POWER_CONTROL")]
    pub do_power_control: bool,
}
