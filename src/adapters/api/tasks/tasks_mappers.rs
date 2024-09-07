use crate::adapters::{api::tasks::tasks_payloads::TaskCreatePayload, api::tasks::tasks_presenters::TaskPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::task_entity::*;

use super::tasks_payloads::{TaskPayload, TaskUpdatePayload};
use super::tasks_presenters::TaskAllPresenter;

pub struct TaskCreatePresenterMapper {}

pub struct TaskUpdatePresenterMapper {}

pub struct TaskAllPresenterMapper {}

pub struct TaskPresenterMapper {}

impl ApiMapper<TaskEntity, TaskPresenter, TaskPayload> for TaskPresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
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

    fn to_entity(_payload: TaskPayload) -> TaskEntity {
        panic!("not implemented");
    }
}


impl ApiMapper<TaskEntity, TaskPresenter, TaskCreatePayload> for TaskCreatePresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
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

    fn to_entity(_payload: TaskCreatePayload) -> TaskEntity {
        panic!("not implemented");
    }
}

impl ApiMapper<TaskEntity, TaskPresenter, TaskUpdatePayload> for TaskUpdatePresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
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

    fn to_entity(_payload: TaskUpdatePayload) -> TaskEntity {
        panic!("not implemented");
    }
}

impl ApiMapper<TaskAllEntity, TaskAllPresenter, TaskPayload> for TaskAllPresenterMapper {
    fn to_api(entity: TaskAllEntity) -> TaskAllPresenter {
        TaskAllPresenter {
            task_id: entity.id,
            title: entity.title,
            description: entity.description,
        }
    }

    fn to_entity(_payload: TaskPayload) -> TaskAllEntity {
        panic!("not implemented");
    }
}
