#[derive(Debug, Clone)]
pub struct TaskEntity {
    pub id: i32,
    pub title: String,
    pub typ: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
}

impl TaskEntity {
    pub fn new(
        id: i32,
        title: String,
        typ: Option<String>,
        priority: Option<String>,
        status: Option<String>,
        description: Option<String>,
        duration: Option<i32>,
        due_date: Option<i64>,
        project_id: Option<i32>,
        task_list: Option<Vec<String>>,
    ) -> Self {
        TaskEntity {
            id,
            title,
            typ,
            priority,
            status,
            description,
            duration,
            due_date,
            project_id,
            task_list,
        }
    }
}
