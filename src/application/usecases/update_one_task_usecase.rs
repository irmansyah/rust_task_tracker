use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskPayload,
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::TaskEntity},
};

pub struct UpdateOneTaskUseCase<'a> {
    task_payload: &'a TaskPayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> UpdateOneTaskUseCase<'a> {
    pub fn new(task_payload: &'a TaskPayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        UpdateOneTaskUseCase { task_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TaskEntity> for UpdateOneTaskUseCase<'a> {
    async fn execute(&self) -> Result<TaskEntity, ApiError> {
        let task = self.repository.update_one_task(&self.task_payload).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot update single task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::application::{repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, usecases::update_one_task_usecase::UpdateOneTaskUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all task tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskPayload::new(1, "This is text task".to_string());
        task_repository
            .expect_update_one_task()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let update_one_task_usecase = UpdateOneTaskUseCase::new(&payload, &task_repository);
        let data = update_one_task_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot update one task", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one task task by id" usecase repo returning one result
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskPayload::new(1, "task 1".to_string());
        task_repository.expect_update_one_task().times(1).returning(|_| {
            Ok(TaskEntity {
                task_id: 1,
                task: String::from("task1"),
            })
        });

        // when calling usecase
        let get_one_task_by_id_usecase = UpdateOneTaskUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.task_id, 1);
        assert_eq!(data.task, "task1");
    }
}
