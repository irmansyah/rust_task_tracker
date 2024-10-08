use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct TaskEntity {
    pub id: String,
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
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl TaskEntity {
    pub fn new(
        id: String,
        user_id: String,
        project_id: String,
        title: String,
        typ: String,
        priority: String,
        status: String,
        description: String,
        duration: i32,
        due_date: i64,
        task_list: Vec<String>,
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    ) -> Self {
        TaskEntity {
            id,
            user_id,
            project_id,
            title,
            typ,
            priority,
            status,
            description,
            duration,
            due_date,
            task_list,
            updated_at,
            created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskAllEntity {
    pub id: String,
    pub user_id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl TaskAllEntity {
    pub fn new(
        id: String,
        user_id: String,
        project_id: String,
        title: String,
        description: String,
        updated_at: NaiveDateTime,
        created_at: NaiveDateTime,
    ) -> Self {
        TaskAllEntity {
            id,
            user_id,
            project_id,
            title,
            description,
            updated_at,
            created_at,
        }
    }
}
