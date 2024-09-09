use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskIdPayload, application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::{error::ApiError, task_entity::TaskEntity}
};

pub struct DeleteOneTaskByIdUseCase<'a> {
    user_payload: &'a TaskIdPayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> DeleteOneTaskByIdUseCase<'a> {
    pub fn new(user_payload: &'a TaskIdPayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
        DeleteOneTaskByIdUseCase { user_payload, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TaskEntity> for DeleteOneTaskByIdUseCase<'a> {
    async fn execute(&self) -> Result<TaskEntity, ApiError> {
        let task = self.repository.delete_task_by_id(&self.user_payload).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot delete single task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    use crate::{adapters::api::users::users_payloads::UserIdPayload, application::repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, domain::task_entity::TaskEntity};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = UserIdPayload::new(String::from("id1"));
        task_repository
            .expect_delete_task_by_id()
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let delete_one_task_by_id_usecase = DeleteOneTaskByIdUseCase::new(&payload, &task_repository);
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
        let payload = UserIdPayload::new(String::from("id1"));
        task_repository.expect_get_task_by_id().times(1).returning(|_| {
            Ok(TaskEntity {
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
            })
        });

        // when calling usecase
        let get_one_task_by_id_usecase = DeleteOneTaskByIdUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, 1);
        assert_eq!(data.title, "task1");
    }
}
