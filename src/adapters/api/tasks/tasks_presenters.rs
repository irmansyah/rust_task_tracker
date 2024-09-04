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
    pub description: String,
}
