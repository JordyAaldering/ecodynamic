mod connection;

use std::{hash::{DefaultHasher, Hash, Hasher}, io, time::Instant};

use ecodynamic_api::*;
use rapl_energy::Rapl;

pub use crate::connection::Connection;

pub fn connect(max_threads: u16) -> io::Result<Connection> {
    Connection::connect(max_threads)
}

pub fn task<'a>(connection: &'a mut Connection, label: &str) -> Task<'a> {
    Task::new(connection, label)
}

pub struct Task<'a> {
    connection: &'a mut Connection,
    task: i32,
    start: Option<(Instant, Rapl)>,
}

impl<'a> Task<'a> {
    pub fn new(connection: &'a mut Connection, label: &str) -> Self {
        Self {
            connection,
            task: hash_label(label),
            start: None,
        }
    }

    pub fn begin(&mut self) -> io::Result<Demand> {
        // Send a signal to the controller that we are at the start of a task
        let request = self.build_request();
        self.connection.write(&request)?;

        // Read the controller's response, which contains the demand for this task
        let demand = self.connection.read();

        // Start measurements for this task
        let rapl = Rapl::new(false).unwrap();
        let now = Instant::now();
        self.start = Some((now, rapl));

        demand
    }

    /// Write measurements to the controller.
    ///
    /// Returns the sample for convenience.
    pub fn end(&mut self) -> io::Result<Sample> {
        let (time, energy) = self.start.take()
            .expect("Task must be started before ending");

        let sample = self.build_sample(time, energy);
        self.connection.write(&sample)?;

        Ok(sample)
    }

    fn build_request(&self) -> Request {
        Request {
            region_uid: self.task,
            problem_size: None,
        }
    }

    fn build_sample(&self, runtime: Instant, energy: Rapl) -> Sample {
        Sample {
            region_uid: self.task,
            runtime: runtime.elapsed().as_secs_f32(),
            energy: energy.elapsed().into_values().sum(),
            usertime: None,
        }
    }
}

fn hash_label(label: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    label.hash(&mut hasher);
    hasher.finish() as i32
}
