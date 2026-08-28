use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Capabilities<'a> {
    app: &'a AppCapabilities,
    ctx: &'a ServerCapabilities,
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

impl<'a> Capabilities<'a> {
    pub fn new(app: &'a AppCapabilities, ctx: &'a ServerCapabilities) -> Self {
        assert!(app.max_threads > 0);
        assert!(ctx.min_power > 0.0 && ctx.min_power <= 1.0);
        assert!(ctx.max_power > 0.0 && ctx.max_power <= 1.0);
        assert!(ctx.min_power <= ctx.max_power);
        Self { app, ctx }
    }

    pub fn pid(&self) -> i32 {
        self.app.pid
    }

    pub fn max_threads(&self) -> u16 {
        self.app.max_threads
    }

    pub fn thread_control(&self) -> bool {
        self.ctx.thread_control
    }

    pub fn pinning_control(&self) -> bool {
        self.ctx.pinning_control
    }

    pub fn power_control(&self) -> bool {
        self.ctx.power_control
    }

    pub fn min_power(&self) -> f32 {
        self.ctx.min_power
    }

    pub fn max_power(&self) -> f32 {
        self.ctx.max_power
    }
}

impl AppCapabilities {
    pub fn new(pid: i32, max_threads: u16) -> Self {
        assert!(max_threads > 0);
        Self { pid, max_threads }
    }
}

impl ServerCapabilities {
    pub fn new(thread_control: bool, pinning_control: bool, power_control: bool, min_power: f32, max_power: f32) -> Self {
        assert!(min_power > 0.0 && min_power <= 1.0);
        assert!(max_power > 0.0 && max_power <= 1.0);
        assert!(min_power <= max_power);
        Self { thread_control, pinning_control, power_control, min_power, max_power }
    }
}
