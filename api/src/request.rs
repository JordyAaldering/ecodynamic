use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    /// A unique identifier of the task we are controlling.
    #[serde(alias = "region_uid")]
    pub task_id: i32,
    /// The same task might have a varying input size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_size: Option<i32>,
}
