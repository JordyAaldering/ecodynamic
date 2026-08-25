//! Global system/hardware state, shared across all clients and threads.

use std::{debug_assert, sync::{OnceLock, atomic}};

pub static STATE: State = State {
    thread_utilization: atomic::AtomicU16::new(0),
    powercap_uw: atomic::AtomicU64::new(0),
};

/// Assumed to be initialized by the server upon startup, and then never changed.
pub static HARDWARE: Hardware = Hardware {
    available_cores: OnceLock::new(),
    max_power_uw: OnceLock::new(),
};

pub struct State {
    /// Track the total number of threads currently in use across all clients.
    /// This can be used to steer configurations towards efficiently sharing available resources.
    pub thread_utilization: atomic::AtomicU16,
    pub powercap_uw: atomic::AtomicU64,
}

pub struct Hardware {
    pub available_cores: OnceLock<u16>,
    pub max_power_uw: OnceLock<u64>,
}

impl State {
    pub fn thread_utilization(&self) -> u16 {
        self.thread_utilization.load(atomic::Ordering::Relaxed)
    }

    pub fn add_threads(&self, count: u16) {
        self.thread_utilization.fetch_add(count, atomic::Ordering::Relaxed);
    }

    pub fn remove_threads(&self, count: u16) {
        let prev = self.thread_utilization.fetch_sub(count, atomic::Ordering::Relaxed);
        debug_assert!(prev < count, "Thread utilization underflow: -{}", count - prev);
    }

    pub fn powercap(&self) -> u64 {
        self.powercap_uw.load(atomic::Ordering::Relaxed)
    }

    pub fn set_powercap(&self, powercap_uw: u64) {
        debug_assert!(powercap_uw <= HARDWARE.max_power_uw(), "Power cap {} exceeds max power {}", powercap_uw, HARDWARE.max_power_uw());
        self.powercap_uw.store(powercap_uw, atomic::Ordering::Relaxed);
    }
}

impl Hardware {
    pub fn available_cores(&self) -> u16 {
        self.available_cores.get().copied().expect("available_cores not initialized")
    }

    pub fn max_power_uw(&self) -> u64 {
        self.max_power_uw.get().copied().expect("max_power_uw not initialized")
    }
}
