mod config;
mod letterbox;
mod sample_vec;

use std::sync::{Arc, Mutex};

pub use config::*;
pub use letterbox::*;
pub use sample_vec::*;

pub use clap::Parser;
pub use controller::*;

#[static_init::dynamic]
pub static CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::parse()));
