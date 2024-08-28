use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::dog_fact_entity::DogFactEntity;
use crate::domain::task_entity::TaskEntity;

use super::models::DogFact;
use super::models::Task;

pub struct DogFactDbMapper {}

pub struct TaskDbMapper {}

impl DbMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_db(entity: DogFactEntity) -> DogFact {
        DogFact {
            id: entity.fact_id,
            fact: entity.fact,
        }
    }

    fn to_entity(model: DogFact) -> DogFactEntity {
        DogFactEntity {
            fact_id: model.id,
            fact: model.fact,
        }
    }
}

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
