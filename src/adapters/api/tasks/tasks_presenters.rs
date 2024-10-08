use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskPresenter {
    pub task_id: String,
    pub user_id: String,
    pub project_id: String,
    pub title: String,
    pub typ: String,
    pub priority: String,
    pub status: String,
    pub description: String,
    pub duration: i32,
    pub due_date: i64,
    pub task_list: Vec<String>,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskAllPresenter {
    pub task_id: String,
    pub user_id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub updated_at: i64,
    pub created_at: i64,
}
