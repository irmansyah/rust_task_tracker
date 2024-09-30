use uuid::Uuid;

use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::*;

use super::task_model::Task;

pub struct TaskDbMapper {}

pub struct TaskAllDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: Uuid::parse_str(&entity.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&entity.user_id).unwrap_or_default(),
            project_id: Uuid::parse_str(&entity.project_id).unwrap_or_default(),
            title: entity.title,
            typ: Some(entity.typ),
            status: Some(entity.status),
            priority: Some(entity.priority),
            description: entity.description,
            duration: Some(entity.duration),
            due_date: Some(entity.due_date),
            task_list: Some(entity.task_list),
            updated_at: todo!(),
            created_at: todo!(),
        }
    }

    fn to_entity(model: Task) -> TaskEntity {
        TaskEntity {
            id: model.id.to_string(),
            user_id: model.user_id.to_string(),
            project_id: model.project_id.to_string(),
            title: model.title,
            typ: model.typ.unwrap_or_default(),
            priority: model.priority.unwrap_or_default(),
            status: model.status.unwrap_or_default(),
            description: model.description,
            duration: model.duration.unwrap_or_default(),
            due_date: model.due_date.unwrap_or_default(),
            task_list: model.task_list.unwrap_or_default(),
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}

impl DbMapper<TaskAllEntity, Task> for TaskAllDbMapper {
    fn to_db(entity: TaskAllEntity) -> Task {
        Task {
            id: Uuid::parse_str(&entity.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&entity.user_id).unwrap_or_default(),
            project_id: Uuid::parse_str(&entity.project_id).unwrap_or_default(),
            title: entity.title,
            description: entity.description,
            typ: todo!(),
            priority: todo!(),
            status: todo!(),
            duration: todo!(),
            due_date: todo!(),
            task_list: todo!(),
            updated_at: todo!(),
            created_at: todo!(),
        }
    }

    fn to_entity(model: Task) -> TaskAllEntity {
        TaskAllEntity {
            id: model.id.to_string(),
            user_id: model.user_id.to_string(),
            project_id: model.project_id.to_string(),
            title: model.title,
            description: model.description,
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}
