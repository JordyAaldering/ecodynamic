use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Sample {
    /// A unique identifier of the parallel region we are controlling.
    pub region_uid: i32,
    /// Total runtime of the previous iteration.
    pub runtime: f32,
    /// Total usertime of the previous iteration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usertime: Option<f32>,
    /// Total energy consumption of the previous iteration.
    pub energy: f32,
}
