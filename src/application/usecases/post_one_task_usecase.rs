use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskPayload,
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::TaskEntity},
};

pub struct PostOneTaskUseCase<'a> {
    task_payload: &'a TaskPayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> PostOneTaskUseCase<'a> {
    pub fn new(task_payload: &'a TaskPayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        PostOneTaskUseCase { task_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TaskEntity> for PostOneTaskUseCase<'a> {
    async fn execute(&self) -> Result<TaskEntity, ApiError> {
        let task = self.repository.post_one_task(&self.task_payload).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot post task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::application::{repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, usecases::post_one_task_usecase::PostOneTaskUseCase};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all task tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskPayload::new(1, Some(String::from("task1")), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!());
        task_repository
            .expect_post_one_task()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let post_one_task_usecase = PostOneTaskUseCase::new(&payload, &task_repository);
        let data = post_one_task_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single task", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one task task by id" usecase repo returning one result
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskPayload::new(1, Some(String::from("task1")), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!());
        task_repository.expect_post_one_task().times(1).returning(|_| Ok(payload));

        // when calling usecase
        let get_one_task_by_id_usecase = PostOneTaskUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, 1);
        assert_eq!(data.title, "task1");
    }
}
