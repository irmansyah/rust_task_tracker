use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::*;
use crate::domain::user_entity::*;

use super::user_model::User;
use super::task_model::Task;

pub struct TaskDbMapper {}

pub struct TaskAllDbMapper {}

pub struct UserDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: entity.id,
            title: entity.title,
            typ: Some(entity.typ),
            status: Some(entity.status),
            priority: Some(entity.priority),
            description: entity.description,
            duration: Some(entity.duration),
            due_date: Some(entity.due_date),
            project_id: Some(entity.project_id),
            task_list: Some(entity.task_list),
        }
    }

    fn to_entity(model: Task) -> TaskEntity {
        TaskEntity {
            id: model.id.to_owned(),
            title: model.title,
            typ: model.typ.unwrap_or_default(),
            priority: model.priority.unwrap_or_default(),
            status: model.status.unwrap_or_default(),
            description: model.description,
            duration: model.duration.unwrap_or_default(),
            due_date: model.due_date.unwrap_or_default(),
            project_id: model.project_id.unwrap_or_default(),
            task_list: model.task_list.unwrap_or_default(),
        }
    }
}

impl DbMapper<TaskAllEntity, Task> for TaskAllDbMapper {
    fn to_db(entity: TaskAllEntity) -> Task {
        Task {
            id: entity.id.clone(),
            title: entity.title,
            description: entity.description,
            typ: todo!(),
            priority: todo!(),
            status: todo!(),
            duration: todo!(),
            due_date: todo!(),
            project_id: todo!(),
            task_list: todo!(),
        }
    }

    fn to_entity(model: Task) -> TaskAllEntity {
        TaskAllEntity {
            id: model.id.to_owned(),
            title: model.title,
            description: model.description,
        }
    }
}

impl DbMapper<UserEntity, User> for UserDbMapper {
    fn to_db(entity: UserEntity) -> User {
        User {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            password: entity.password,
        }
    }

    fn to_entity(model: User) -> UserEntity {
        UserEntity {
            id: model.id.to_owned(),
            username: model.username,
            email: model.email,
            password: model.password,
        }
    }
}
