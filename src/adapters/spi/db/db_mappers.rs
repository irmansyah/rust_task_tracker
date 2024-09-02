use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::TaskEntity;

use super::models::Task;

pub struct TaskDbMapper {}

impl DbMapper<TaskEntity, Task> for TaskDbMapper {
    fn to_db(entity: TaskEntity) -> Task {
        Task {
            id: entity.task_id,
            task: entity.task,
        }
    }

    fn to_entity(model: Task) -> TaskEntity {
        TaskEntity {
            task_id: model.id,
            task: model.task,
        }
    }
}
