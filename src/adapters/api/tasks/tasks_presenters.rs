use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskPresenter {
    pub task_id: i32,
    pub title: String,
    pub typ: String,
    pub priority: String,
    pub status: String,
    pub description: String,
    pub duration: i32,
    pub due_date: i64,
    pub project_id: i32,
    pub task_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskAllPresenter {
    pub task_id: i32,
    pub title: String,
    pub description: String,
}
