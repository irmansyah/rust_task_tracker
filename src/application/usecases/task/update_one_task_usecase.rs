use async_trait::async_trait;

use crate::{
    adapters::api::tasks::tasks_payloads::TaskUpdatePayload,
    application::{repositories::tasks_repository_abstract::TasksRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, task_entity::TaskEntity},
};

pub struct UpdateOneTaskUseCase<'a> {
    task_payload: &'a TaskUpdatePayload,
    repository: &'a dyn TasksRepositoryAbstract,
}

impl<'a> UpdateOneTaskUseCase<'a> {
    pub fn new(task_payload: &'a TaskUpdatePayload, repository: &'a dyn TasksRepositoryAbstract) -> Self {
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

    use crate::{adapters::api::tasks::tasks_payloads::*, application::{repositories::tasks_repository_abstract::MockTasksRepositoryAbstract, usecases::task::update_one_task_usecase::UpdateOneTaskUseCase}};

    #[actix_rt::test]
    async fn test_should_return_generic_message_when_unexpected_repo_error() {
        // given the "all task tasks" usecase repo with an unexpected random error
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskUpdatePayload::new(
            String::from("id1"),
            Some(String::from("id1")),
            Some(String::from("task1")),
            Some(TaskTypePayload::Work),
            Some(TaskPriorityPayload::Low),
            Some(TaskStatusPayload::ToDo(TaskStatusToDoPayload::NotStarted)),
            Some(String::from("")),
            Some(1),
            Some(321472382),
            Some(1),
            Some([].to_vec()),
        );
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
        // given the "one task by id" usecase repo returning one result
        let mut task_repository = MockTasksRepositoryAbstract::new();
        let payload = TaskUpdatePayload::new(
            String::from("id1"),
            Some(String::from("id1")),
            Some(String::from("task1")),
            Some(TaskTypePayload::Work),
            Some(TaskPriorityPayload::Low),
            Some(TaskStatusPayload::ToDo(TaskStatusToDoPayload::NotStarted)),
            Some(String::from("")),
            Some(1),
            Some(321472382),
            Some(1),
            Some([].to_vec()),
        );
        task_repository.expect_update_one_task().times(1).returning(|_| {
            Ok(TaskEntity {
                id: String::from("id1"),
                user_id: String::from("id1"),
                title: String::from("task1"),
                typ: TaskTypePayload::Work.to_string(),
                priority: TaskPriorityPayload::Low.to_string(),
                status: TaskStatusPayload::ToDo(TaskStatusToDoPayload::NotStarted).to_string(),
                description: String::from(""),
                duration: 1,
                due_date: 321472382,
                project_id: 1,
                task_list: [].to_vec(),
                updated_at: todo!(),
                created_at: todo!(),
            })
        });

        // when calling usecase
        let get_one_task_by_id_usecase = UpdateOneTaskUseCase::new(&payload, &task_repository);
        let data = get_one_task_by_id_usecase.execute().await.unwrap();

        // then assert the result is the expected entity
        assert_eq!(data.id, String::from("id1"));
        assert_eq!(data.title, "task1");
    }
}
