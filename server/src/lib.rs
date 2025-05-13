mod config;
mod letterbox;
mod sample_vec;

pub use config::*;
pub use letterbox::*;
pub use sample_vec::*;

pub use clap::Parser;
pub use controller::*;

#[static_init::dynamic]
pub static CONFIG: SharedConfig = SharedConfig::new(Config::parse());
