use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskPresenter {
    pub task_id: i32,
    pub txt: String,
}
