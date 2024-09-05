use crate::adapters::{api::tasks::tasks_payloads::TaskPayload, api::tasks::tasks_presenters::TaskPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::task_entity::TaskEntity;

pub struct TaskPresenterMapper {}

impl ApiMapper<TaskEntity, TaskPresenter, TaskPayload> for TaskPresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
            title: entity.title,
            typ: entity.typ.unwrap(),
            priority: entity.priority.unwrap(),
            status: entity.status.unwrap(),
            description: entity.description.unwrap(),
            duration: entity.duration.unwrap_or_default(),
            due_date: entity.due_date.unwrap_or_default(),
            project_id: entity.project_id.unwrap_or_default(),
            task_list: entity.task_list.unwrap_or_default(),
        }
    }

    fn to_entity(_payload: TaskPayload) -> TaskEntity {
        panic!("not implemented");
    }
}
