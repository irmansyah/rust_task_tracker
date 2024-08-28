use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskPayload {
    // implement for POST/UPDATE requests
    pub task_id: i32,
    pub task: String,
}

impl TaskPayload {
    pub fn new(task_id: i32, task: String) -> Self {
        TaskPayload { task_id, task }
    }
}
