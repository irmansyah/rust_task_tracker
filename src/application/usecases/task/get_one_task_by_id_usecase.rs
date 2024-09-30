use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskDataPayload,
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::TaskEntity},
};

pub struct GetOneTaskByIdUseCase<'a> {
    task_payload: &'a TaskDataPayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> GetOneTaskByIdUseCase<'a> {
    pub fn new(task_payload: &'a TaskDataPayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        GetOneTaskByIdUseCase { task_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TaskEntity> for GetOneTaskByIdUseCase<'a> {
    async fn execute(&self) -> Result<TaskEntity, ApiError> {
        let task = self.repository.get_task_by_id(&self.task_payload).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get single task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{
        adapters::api::{tasks::tasks_payloads::TaskDataPayload, tasks::tasks_payloads::*},
        application::repositories::tasks_repository_abstract::MockTasksRepositoryAbstract,
        domain::task_entity::TaskEntity,
    };

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskDataPayload::new(Some(String::from("id1")), Some(String::from("id1")));
        task_repository
            .expect_get_task_by_id()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single task", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one task by id" usecase repo returning one result
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskDataPayload::new(Some(String::from("id1")), Some(String::from("id1")));
        task_repository.expect_get_task_by_id().times(1).returning(|_| {
            Ok(TaskEntity {
                id: String::from("id1"),
                user_id: String::from("id1"),
                project_id: String::from("id1"),
                title: String::from("task1"),
                typ: TaskTypePayload::Work.to_string(),
                priority: TaskPriorityPayload::Low.to_string(),
                status: TaskStatusPayload::ToDo(TaskStatusToDoPayload::NotStarted).to_string(),
                description: String::from(""),
                duration: 1,
                due_date: 321472382,
                task_list: [].to_vec(),
                updated_at: todo!(),
                created_at: todo!(),
            })
        });

        // when calling usecase
        let get_one_task_by_id_usecase = GetOneTaskByIdUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, String::from("id1"));
        assert_eq!(data.title, "task1");
    }
}
