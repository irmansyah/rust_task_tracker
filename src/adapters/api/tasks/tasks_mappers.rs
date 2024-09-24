use chrono::NaiveDateTime;

use crate::adapters::{api::tasks::tasks_payloads::TaskCreatePayload, api::tasks::tasks_presenters::TaskPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::task_entity::*;

use super::tasks_payloads::{TaskIdPayload, TaskPayload, TaskUpdatePayload};
use super::tasks_presenters::TaskAllPresenter;

pub struct TaskCreatePresenterMapper {}

pub struct TaskUpdatePresenterMapper {}

pub struct TaskAllPresenterMapper {}

pub struct TaskPresenterMapper {}

impl ApiMapper<TaskEntity, TaskPresenter, TaskIdPayload> for TaskPresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
            user_id: entity.user_id,
            title: entity.title,
            typ: entity.typ,
            priority: entity.priority,
            status: entity.status,
            description: entity.description,
            duration: entity.duration,
            due_date: entity.due_date,
            project_id: entity.project_id,
            task_list: entity.task_list,
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
        }
    }

    fn to_entity(_payload: TaskIdPayload) -> TaskEntity {
        panic!("not implemented");
    }
}

impl ApiMapper<TaskEntity, TaskPresenter, TaskCreatePayload> for TaskCreatePresenterMapper {
    fn to_api(entity: TaskEntity) -> TaskPresenter {
        TaskPresenter {
            task_id: entity.id,
            user_id: entity.user_id,
            title: entity.title,
            typ: entity.typ,
            priority: entity.priority,
            status: entity.status,
            description: entity.description,
            duration: entity.duration,
            due_date: entity.due_date,
            project_id: entity.project_id,
            task_list: entity.task_list,
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
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
            user_id: entity.user_id,
            title: entity.title,
            typ: entity.typ,
            priority: entity.priority,
            status: entity.status,
            description: entity.description,
            duration: entity.duration,
            due_date: entity.due_date,
            project_id: entity.project_id,
            task_list: entity.task_list,
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
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
            user_id: entity.user_id,
            title: entity.title,
            description: entity.description,
            updated_at: naive_datetime_to_unixtimemillis(entity.updated_at),
            created_at: naive_datetime_to_unixtimemillis(entity.created_at),
        }
    }

    fn to_entity(_payload: TaskPayload) -> TaskAllEntity {
        panic!("not implemented");
    }
}

fn naive_datetime_to_unixtimemillis(datetime: NaiveDateTime) -> i64 {
    // Get the Unix timestamp in seconds and convert to milliseconds
    let millis = datetime.and_utc().timestamp_millis();
    millis
}
