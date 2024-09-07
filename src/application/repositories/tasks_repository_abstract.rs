use async_trait::async_trait;

use crate::{adapters::api::tasks::tasks_payloads::*, domain::task_entity::*};

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait TasksRepositoryAbstract {
    async fn post_one_task(&self, task_payload: &TaskCreatePayload) -> Result<TaskEntity, Box<dyn Error>>;
    async fn update_one_task(&self, task_payload: &TaskUpdatePayload) -> Result<TaskEntity, Box<dyn Error>>;
    async fn get_task_by_id(&self, task_id: i32) -> Result<TaskEntity, Box<dyn Error>>;
    async fn get_all_tasks(&self) -> Result<Vec<TaskAllEntity>, Box<dyn Error>>;
    async fn delete_task_by_id(&self, task_id: i32) -> Result<TaskEntity, Box<dyn Error>>;
}
