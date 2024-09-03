use async_trait::async_trait;

use crate::{
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{task_entity::TaskEntity, error::ApiError},
};

pub struct DeleteOneTaskByIdUseCase<'a> {
    task_id: &'a i32,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> DeleteOneTaskByIdUseCase<'a> {
    pub fn new(task_id: &'a i32, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        DeleteOneTaskByIdUseCase { task_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TaskEntity> for DeleteOneTaskByIdUseCase<'a> {
    async fn execute(&self) -> Result<TaskEntity, ApiError> {
        let task = self.repository.delete_task_by_id(*self.task_id).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot delete single task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    use crate::{application::repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, domain::task_entity::TaskEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        task_repository
            .expect_delete_task_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let delete_one_task_by_id_usecase = DeleteOneTaskByIdUseCase::new(&1, &task_repository);
        let data = delete_one_task_by_id_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get single dog task", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_one_result() {
        // given the "one dog task by id" usecase repo returning one result
        let mut task_repository = MockTasksRepositoryAbstract::new();
        task_repository.expect_get_task_by_id().with(eq(1)).times(1).returning(|_| {
            Ok(TaskEntity {
                task_id: 1,
                task: String::from("task1"),
            })
        });

        // when calling usecase
        let get_one_task_by_id_usecase = DeleteOneTaskByIdUseCase::new(&1, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.task_id, 1);
        assert_eq!(data.task, "task1");
    }
}
