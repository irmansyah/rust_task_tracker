use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::TaskEntity;
use crate::domain::user_entity::UserEntity;

use super::user_model::User;
use super::task_model::Task;

pub struct TaskDbMapper {}

pub struct UserDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: entity.id,
            title: Some(entity.title),
            typ: entity.typ,
            status: entity.status,
            priority: entity.priority,
            description: entity.description,
            duration: entity.duration,
            due_date: entity.due_date,
            project_id: entity.project_id,
            task_list: entity.task_list,
        }
    }

    fn to_entity(model: Task) -> TaskEntity {
        TaskEntity {
            id: model.id.to_owned(),
            title: model.title.unwrap(),
            typ: model.typ,
            priority: model.priority,
            status: model.status,
            description: model.description,
            duration: model.duration,
            due_date: model.due_date,
            project_id: model.project_id,
            task_list: model.task_list,
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
