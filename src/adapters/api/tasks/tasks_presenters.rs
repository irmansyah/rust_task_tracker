use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TaskPresenter<T> {
//     pub code: i32,
//     pub message: String,
//     pub data: T,
// }

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
