use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Capabilities<'a> {
    app: &'a AppCapabilities,
    ctx: &'a ServerCapabilities,
    hw: &'a HardwareCapabilities,
}

/// Represents the capabilities of an application.
///
/// These are provided by the application as JSON.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapabilities {
    /// The process ID of the application.
    pid: i32,

    /// Maximum number of threads the application may use.
    max_threads: u16,
}

/// Represents the configuration of the resource controller and the capabilities of the hardware.
///
/// These are configured by the user as command line arguments.
#[derive(Clone, Debug, clap::Parser)]
pub struct ServerCapabilities {
    /// Describes the importance of optimising for energy efficiency over runtime performance.
    /// A value of 1 means that only energy efficiency is optimised for, while a value of 0 means that only runtime performance is optimised for.
    ///
    /// Range: [0,1]
    #[clap(short('a'), long, default_value_t = 0.9)]
    energy_preference: f32,

    #[clap(long)]
    thread_control: bool,

    #[clap(long)]
    pinning_control: bool,

    #[clap(long)]
    power_control: bool,

    /// Minimum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[clap(long, default_value_t = 0.1)]
    min_power: f32,

    /// Maximum allowed percentage of the powercap.
    ///
    /// Range: (0,1]
    #[clap(long, default_value_t = 1.0)]
    max_power: f32,
}

/// Represents the capabilities of the hardware.
///
/// These are derived on startup by the server.
#[derive(Clone, Debug)]
pub struct HardwareCapabilities {
    available_threads: u16,
    max_power_uw: u64,
}

impl<'a> Capabilities<'a> {
    pub fn new(app: &'a AppCapabilities, ctx: &'a ServerCapabilities, hw: &'a HardwareCapabilities) -> Self {
        assert!(app.max_threads > 0);
        assert!(ctx.energy_preference >= 0.0 && ctx.energy_preference <= 1.0);
        assert!(ctx.min_power > 0.0 && ctx.min_power <= 1.0);
        assert!(ctx.max_power > 0.0 && ctx.max_power <= 1.0);
        assert!(ctx.min_power <= ctx.max_power);
        Self { app, ctx, hw }
    }

    pub fn pid(&self) -> i32 { self.app.pid }
    pub fn max_threads(&self) -> u16 { self.app.max_threads }
    pub fn energy_preference(&self) -> f32 { self.ctx.energy_preference }
    pub fn thread_control(&self) -> bool { self.ctx.thread_control }
    pub fn pinning_control(&self) -> bool { self.ctx.pinning_control }
    pub fn power_control(&self) -> bool { self.ctx.power_control }
    pub fn min_power(&self) -> f32 { self.ctx.min_power }
    pub fn max_power(&self) -> f32 { self.ctx.max_power }
    pub fn available_threads(&self) -> u16 { self.hw.available_threads }
    pub fn max_power_uw(&self) -> u64 { self.hw.max_power_uw }
}

impl AppCapabilities {
    pub fn new(pid: i32, max_threads: u16) -> Self {
        assert!(max_threads > 0);
        Self { pid, max_threads }
    }
}

impl ServerCapabilities {
    pub fn new(energy_preference: f32, thread_control: bool, pinning_control: bool, power_control: bool, min_power: f32, max_power: f32) -> Self {
        assert!(energy_preference >= 0.0 && energy_preference <= 1.0);
        assert!(min_power > 0.0 && min_power <= 1.0);
        assert!(max_power > 0.0 && max_power <= 1.0);
        assert!(min_power <= max_power);
        Self { energy_preference, thread_control, pinning_control, power_control, min_power, max_power }
    }
}

impl HardwareCapabilities {
    pub fn new(available_threads: u16, max_power_uw: u64) -> Self {
        Self { available_threads, max_power_uw }
    }
}
