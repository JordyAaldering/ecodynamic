use std::{hash::{DefaultHasher, Hash, Hasher}, io, time::Instant};

use controller::{Demand, Request, Sample};
use rapl_energy::Rapl;

use crate::Connection;

pub struct Region<'a> {
    connection: &'a mut Connection,
    region_uid: i32,
    start: Option<(Instant, Rapl)>,
}

impl<'a> Region<'a> {
    pub fn new(connection: &'a mut Connection, label: &str) -> Self {
        Self {
            connection,
            region_uid: hash_label(label),
            start: None,
        }
    }

    pub fn begin(&mut self) -> io::Result<Demand> {
        // Send a signal to the controller that we are at the start of a parallel region
        let request = self.build_request();
        self.connection.write(&request)?;

        // Read the controller's response, which contains the demand for this region
        let demand = self.connection.read();

        // Start measurements for this region
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
            .expect("Region must be started before ending");

        let sample = self.build_sample(time, energy);
        self.connection.write(&sample)?;

        Ok(sample)
    }

    fn build_request(&self) -> Request {
        Request {
            region_uid: self.region_uid,
            problem_size: None,
        }
    }

    fn build_sample(&self, runtime: Instant, energy: Rapl) -> Sample {
        Sample {
            region_uid: self.region_uid,
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
