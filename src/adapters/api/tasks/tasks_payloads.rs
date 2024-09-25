use std::fmt;

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskTypePayload {
    Personal,
    Work,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskPriorityPayload {
    Low,
    Medium,
    High,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatusToDoPayload {
    NotStarted,
    Document,
    Bug,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatusInProgressPayload {
    Doing,
    Testing,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatusPayload {
    ToDo(TaskStatusToDoPayload),
    InProgress(TaskStatusInProgressPayload),
    Completed,
    None,
}

impl Default for TaskTypePayload {
    fn default() -> Self {
        TaskTypePayload::None
    }
}

impl Default for TaskPriorityPayload {
    fn default() -> Self {
        TaskPriorityPayload::None
    }
}

impl Default for TaskStatusPayload {
    fn default() -> Self {
        TaskStatusPayload::None
    }
}

impl Default for TaskStatusToDoPayload {
    fn default() -> Self {
        TaskStatusToDoPayload::None
    }
}

impl Default for TaskStatusInProgressPayload {
    fn default() -> Self {
        TaskStatusInProgressPayload::None
    }
}

impl fmt::Display for TaskTypePayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskTypePayload::Personal => write!(f, "Personal"),
            TaskTypePayload::Work => write!(f, "Work"),
            TaskTypePayload::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for TaskPriorityPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriorityPayload::Low => write!(f, "Low"),
            TaskPriorityPayload::Medium => write!(f, "Medium"),
            TaskPriorityPayload::High => write!(f, "High"),
            TaskPriorityPayload::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for TaskStatusPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatusPayload::ToDo(status) => write!(f, "ToDo: ({})", status),
            TaskStatusPayload::InProgress(status) => write!(f, "In Progress: ({})", status),
            TaskStatusPayload::Completed => write!(f, "Completed"),
            TaskStatusPayload::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for TaskStatusToDoPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatusToDoPayload::NotStarted => write!(f, "Not Started"),
            TaskStatusToDoPayload::Document => write!(f, "Document"),
            TaskStatusToDoPayload::Bug => write!(f, "Bug"),
            TaskStatusToDoPayload::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for TaskStatusInProgressPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatusInProgressPayload::Doing => write!(f, "Doing"),
            TaskStatusInProgressPayload::Testing => write!(f, "Testing"),
            TaskStatusInProgressPayload::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskIdPayload {
    pub task_id: String,
}

impl TaskIdPayload {
    pub fn new(task_id: String) -> Self {
        TaskIdPayload { task_id }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TaskDataPayload {
    pub task_id: Option<String>,
    pub user_id: Option<String>,
}

impl TaskDataPayload {
    pub fn new(task_id: Option<String>, user_id: Option<String>) -> Self {
        TaskDataPayload { task_id, user_id }
    }

    pub fn from_option(option: Option<web::Json<TaskDataPayload>>) -> Self {
        option.map_or_else(|| TaskDataPayload::default(), |json| json.into_inner())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskCreatePayload {
    pub user_id: Option<String>,
    pub title: String,
    pub typ: Option<TaskTypePayload>,
    pub priority: Option<TaskPriorityPayload>,
    pub status: Option<TaskStatusPayload>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
}

impl TaskCreatePayload {
    pub fn new(
        user_id: Option<String>,
        title: String,
        typ: Option<TaskTypePayload>,
        priority: Option<TaskPriorityPayload>,
        status: Option<TaskStatusPayload>,
        description: Option<String>,
        duration: Option<i32>,
        due_date: Option<i64>,
        project_id: Option<i32>,
        task_list: Option<Vec<String>>,
    ) -> Self {
        TaskCreatePayload {
            user_id,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskUpdatePayload {
    pub task_id: String,
    pub user_id: Option<String>,
    pub title: Option<String>,
    pub typ: Option<TaskTypePayload>,
    pub priority: Option<TaskPriorityPayload>,
    pub status: Option<TaskStatusPayload>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
}

impl TaskUpdatePayload {
    pub fn new(
        task_id: String,
        user_id: Option<String>,
        title: Option<String>,
        typ: Option<TaskTypePayload>,
        priority: Option<TaskPriorityPayload>,
        status: Option<TaskStatusPayload>,
        description: Option<String>,
        duration: Option<i32>,
        due_date: Option<i64>,
        project_id: Option<i32>,
        task_list: Option<Vec<String>>,
    ) -> Self {
        TaskUpdatePayload {
            task_id,
            user_id,
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

pub struct TaskPayload {}
