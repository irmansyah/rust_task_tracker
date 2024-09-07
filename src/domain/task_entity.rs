#[derive(Debug, Clone)]
pub struct TaskEntity {
    pub id: i32,
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

impl TaskEntity {
    pub fn new(
        id: i32,
        title: String,
        typ: String,
        priority: String,
        status: String,
        description: String,
        duration: i32,
        due_date: i64,
        project_id: i32,
        task_list: Vec<String>,
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

#[derive(Debug, Clone)]
pub struct TaskAllEntity {
    pub id: i32,
    pub title: String,
    pub description: String,
}

impl TaskAllEntity {
    pub fn new(
        id: i32,
        title: String,
        description: String,
    ) -> Self {
        TaskAllEntity {
            id,
            title,
            description,
        }
    }
}
