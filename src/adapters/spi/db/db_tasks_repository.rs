use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

use crate::adapters::api::tasks::tasks_payloads::TaskPayload;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::TaskDbMapper, schema::tasks::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::tasks_repository_abstract::TasksRepositoryAbstract};
use crate::domain::task_entity::TaskEntity;

use crate::adapters::spi::db::schema::tasks;

use super::models::{NewTask, Task};

pub struct TasksRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl TasksRepositoryAbstract for TasksRepository {
    async fn post_one_task(&self, task_payload: &TaskPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_title = task_payload.title.clone().unwrap_or_default();
        let data_typ = task_payload.typ.clone().unwrap_or_default().to_string();
        let data_priority = task_payload.priority.clone().unwrap_or_default().to_string();
        let data_status = task_payload.status.clone().unwrap_or_default().to_string();
        let data_description = task_payload.description.clone().unwrap_or_default();
        let data_duration = task_payload.duration.unwrap_or_default();
        let data_due_date = task_payload.due_date.unwrap_or_default();
        let data_project_id = task_payload.project_id.unwrap_or_default();
        let data_task_list: Option<Vec<&str>> = task_payload.task_list.as_ref().map(|vec| vec.iter().map(|s| s.as_str()).collect());

        let new_task = NewTask {
            title: Some(&data_title),
            typ: Some(&data_typ),
            priority: Some(&data_priority),
            status: Some(&data_status),
            description: Some(&data_description),
            duration: Some(data_duration),
            due_date: Some(data_due_date),
            project_id: Some(data_project_id),
            task_list: data_task_list,
        };

        let result = diesel::insert_into(tasks::table).values(&new_task).returning(Task::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_task(&self, task_payload: &TaskPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let data_task_list: Option<Vec<&str>> = task_payload.task_list.as_ref().map(|vec| vec.iter().map(|s| s.as_str()).collect());
        let target = tasks.filter(id.eq(task_payload.task_id));
        let result = diesel::update(target)
            .set((
                task_payload.title.clone().map(|data| title.eq(data)),
                task_payload.typ.clone().map(|data| typ.eq(data.to_string())),
                task_payload.priority.clone().map(|data| priority.eq(data.to_string())),
                task_payload.status.clone().map(|data| priority.eq(data.to_string())),
                task_payload.description.clone().map(|data| description.eq(data)),
                task_payload.duration.map(|data| duration.eq(data)),
                task_payload.due_date.map(|data| due_date.eq(data)),
                task_payload.project_id.map(|data| project_id.eq(data)),
                data_task_list.map(|data| task_list.eq(data)),
            ))
            .returning(Task::as_returning())
            .get_result(&mut conn);

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

    async fn delete_task_by_id(&self, task_id: i32) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let target_task = tasks::table.filter(tasks::id.eq(task_id));
        let result = diesel::delete(target_task).get_result::<Task>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
