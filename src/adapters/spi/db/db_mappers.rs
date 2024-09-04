use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::TaskEntity;

use super::models::Task;

pub struct TaskDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: entity.id,
            title: entity.title,
            typ: entity.typ,
            priority: entity.priority,
            status: entity.status,
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
            title: model.title,
            description: model.description,
            typ: todo!(),
            priority: todo!(),
            status: todo!(),
            duration: todo!(),
            due_date: todo!(),
            project_id: todo!(),
            task_list: todo!(),
        }
    }
}
