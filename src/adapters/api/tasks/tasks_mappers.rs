use crate::adapters::{api::tasks::tasks_payloads::TaskPayload, api::tasks::tasks_presenters::TaskPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::task_entity::TaskEntity;

pub struct TaskPresenterMapper {}

impl ApiMapper<TaskEntity, TaskPresenter, TaskPayload> for TaskPresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.task_id,
            txt: entity.task,
        }
    }

    fn to_entity(_payload: TaskPayload) -> TaskEntity {
        panic!("not implemented");
    }
}
