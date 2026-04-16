use std::mem;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// The same region might have a varying input size.
    pub problem_size: i32,
}
