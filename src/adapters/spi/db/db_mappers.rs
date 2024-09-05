use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::TaskEntity;

use super::models::Task;

pub struct TaskDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: entity.id,
            title: Some(entity.title),
            typ: entity.typ,
            status: entity.status,
            priority: entity.priority,
            description: entity.description,
            duration: todo!(),
            due_date: todo!(),
            project_id: todo!(),
            task_list: todo!(),
            // typ: todo!(),
            // status: todo!(),
            // duration: todo!(),
            // due_date: todo!(),
            // project_id: todo!(),
            // task_list: todo!(),
            // typ: entity.typ,
            // priority: entity.priority,
            // status: entity.status,
            // duration: entity.duration,
            // due_date: entity.due_date,
            // project_id: entity.project_id,
            // task_list: entity.task_list,
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
