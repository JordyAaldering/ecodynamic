//! Global system/hardware state, shared across all clients and threads.

use std::sync::atomic::AtomicU16;

/// Track the total number of threads currently in use across all clients.
/// This can be used to steer configurations towards efficiently sharing available resources.
pub static GLOBAL_THREAD_COUNT: AtomicU16 = AtomicU16::new(0);

/// Temporary value, until I convert this to a proper configuration parameter.
pub(crate) const AVAILABLE_CORES: u16 = 16;
