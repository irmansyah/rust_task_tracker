#[derive(Debug, Clone)]
pub struct TaskEntity {
    pub task_id: i32,
    pub task: String,
}

impl TaskEntity {
    pub fn new(task_id: i32, task: String) -> Self {
        TaskEntity { task_id, task }
    }
}
