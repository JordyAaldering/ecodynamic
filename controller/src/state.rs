//! Global system state, shared across all clients and threads.

use std::sync::atomic;

pub static STATE: State = State {
    thread_utilization: atomic::AtomicU16::new(0),
    powercap_uw: atomic::AtomicU64::new(0),
};

pub struct State {
    /// Track the total number of threads currently in use across all clients.
    /// This can be used to steer configurations towards efficiently sharing available resources.
    pub thread_utilization: atomic::AtomicU16,
    pub powercap_uw: atomic::AtomicU64,
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
        debug_assert!(count > prev, "Thread utilization underflow: -{}", count - prev);
    }

    pub fn powercap(&self) -> u64 {
        self.powercap_uw.load(atomic::Ordering::Relaxed)
    }

    pub fn set_powercap(&self, powercap_uw: u64) {
        self.powercap_uw.store(powercap_uw, atomic::Ordering::Relaxed);
    }
}
