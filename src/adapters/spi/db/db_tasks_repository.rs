use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

use crate::adapters::api::tasks::tasks_payloads::TaskPayload;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::TaskDbMapper, models::Task, schema::tasks::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::tasks_repository_abstract::TasksRepositoryAbstract};
use crate::domain::task_entity::TaskEntity;

pub struct TasksRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl TasksRepositoryAbstract for TasksRepository {
    async fn post_one_task(&self, task_payload: &TaskPayload) -> Result<TaskEntity, Box<dyn Error>> {

        let new_task = Task::new(task_payload.task_id, task_payload.task);
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let mut conn = self.db_connection.create_task(new_task).expect("couldn't get db connection from pool");

        let new_task = Task::new(task_payload.task_id.clone(), task_payload.task.clone());

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_task_by_id(&self, task_id: i32) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = tasks.filter(id.eq(task_id)).get_result::<Task>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_tasks(&self) -> Result<Vec<TaskEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = tasks.load::<Task>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(TaskDbMapper::to_entity).collect::<Vec<TaskEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
