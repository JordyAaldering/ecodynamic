use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// The same region might have a varying input size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_size: Option<i32>,
}
