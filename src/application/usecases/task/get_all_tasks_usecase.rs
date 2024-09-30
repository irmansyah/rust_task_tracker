use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskDataPayload,
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::*},
};

pub struct GetAllTasksUseCase<'a> {
    task_payload: &'a TaskDataPayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> GetAllTasksUseCase<'a> {
    pub fn new(task_payload: &'a TaskDataPayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        GetAllTasksUseCase { task_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<TaskAllEntity>> for GetAllTasksUseCase<'a> {
    async fn execute(&self) -> Result<Vec<TaskAllEntity>, ApiError> {
        let tasks = self.repository.get_all_tasks(&self.task_payload).await;

        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all tasks", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{application::repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, domain::task_entity::TaskAllEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskDataPayload::new(Some(String::from("id1")), Some(String::from("id1")));
        task_repository
            .expect_get_all_tasks()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&payload, &task_repository);
        let data = get_all_tasks_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all tasks", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all tasks" usecase repo returning an empty list
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskDataPayload::new(Some(String::from("id1")), Some(String::from("id1")));
        task_repository.expect_get_all_tasks().times(1).returning(|_| Ok(Vec::<TaskAllEntity>::new()));

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&payload, &task_repository);
        let data = get_all_tasks_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all tasks" usecase repo returning a list of 2 entities
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskDataPayload::new(Some(String::from("id1")), Some(String::from("id1")));
        task_repository.expect_get_all_tasks().times(1).returning(|_| {
            Ok(vec![
                TaskAllEntity {
                    id: String::from("id1"),
                    user_id: String::from("id1"),
                    title: String::from("Task 1"),
                    project_id: String::from("id1"),
                    description: todo!(),
                    updated_at: todo!(),
                    created_at: todo!(),
                },
                TaskAllEntity {
                    id: String::from("id2"),
                    user_id: String::from("id1"),
                    title: String::from("Task 2"),
                    project_id: String::from("id1"),
                    description: todo!(),
                    updated_at: todo!(),
                    created_at: todo!(),
                },
            ])
        });

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&payload, &task_repository);
        let data = get_all_tasks_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
