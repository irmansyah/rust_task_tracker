use async_trait::async_trait;

use crate::{
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::TaskEntity},
};

pub struct GetAllTasksUseCase<'a> {
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> GetAllTasksUseCase<'a> {
    pub fn new(repository: &'a dyn TasksRepositoryAbstract) -> Self {
        GetAllTasksUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<TaskEntity>> for GetAllTasksUseCase<'a> {
    async fn execute(&self) -> Result<Vec<TaskEntity>, ApiError> {
        let tasks = self.repository.get_all_tasks().await;

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

    use crate::{application::repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, domain::task_entity::TaskEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        task_repository
            .expect_get_all_tasks()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_repository);
        let data = get_all_tasks_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog tasks", result.message);
    }

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog tasks" usecase repo returning an empty list
        let mut task_repository = MockTasksRepositoryAbstract::new();
        task_repository.expect_get_all_tasks().with().times(1).returning(|| Ok(Vec::<TaskEntity>::new()));

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_repository);
        let data = get_all_tasks_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog tasks" usecase repo returning a list of 2 entities
        let mut task_repository = MockTasksRepositoryAbstract::new();
        task_repository.expect_get_all_tasks().with().times(1).returning(|| {
            Ok(vec![
                TaskEntity {
                    id: 1,
                    title: String::from("task1"),
                    typ: todo!(),
                    priority: todo!(),
                    status: todo!(),
                    description: todo!(),
                    duration: todo!(),
                    due_date: todo!(),
                    project_id: todo!(),
                    task_list: todo!(),
                },
                TaskEntity {
                    id: 2,
                    title: String::from("task1"),
                    typ: todo!(),
                    priority: todo!(),
                    status: todo!(),
                    description: todo!(),
                    duration: todo!(),
                    due_date: todo!(),
                    project_id: todo!(),
                    task_list: todo!(),
                },
            ])
        });

        // when calling usecase
        let get_all_tasks_usecase = GetAllTasksUseCase::new(&task_repository);
        let data = get_all_tasks_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}
