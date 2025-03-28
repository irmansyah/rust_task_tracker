use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::task_entity::*;
use crate::{adapters::api::tasks::tasks_payloads::*, application::repositories::tasks_repository_abstract::TasksRepositoryAbstract};

use super::db_tasks_mappers::{TaskAllDbMapper, TaskDbMapper};
use super::schema::tasks::{self, *};
use super::task_model::*;
use crate::adapters::spi::db::{db_connection::DbConnection, schema::tasks::dsl::*};

pub struct TasksRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl TasksRepositoryAbstract for TasksRepository {
    async fn post_one_task(&self, task_payload: &TaskCreatePayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let data_user_id_uuid = Uuid::parse_str(&task_payload.user_id.clone().unwrap_or_default()).unwrap_or_default();
        let data_project_id_uuid = Uuid::parse_str(&task_payload.project_id.clone().unwrap_or_default()).unwrap_or_default();
        let data_title = task_payload.title.clone();
        let data_typ = task_payload.typ.clone().unwrap_or_default().to_string();
        let data_priority = task_payload.priority.clone().unwrap_or_default().to_string();
        let data_status = task_payload.status.clone().unwrap_or_default().to_string();
        let data_description = task_payload.description.clone().unwrap_or_default();
        let data_duration = task_payload.duration.unwrap_or_default();
        let data_due_date = task_payload.due_date.unwrap_or_default();
        let data_task_list: Option<Vec<&str>> = task_payload.task_list.as_ref().map(|vec| vec.iter().map(|s| s.as_str()).collect());

        let new_task = TaskNew {
            user_id: &data_user_id_uuid,
            project_id: &data_project_id_uuid,
            title: &data_title,
            typ: &data_typ,
            priority: &data_priority,
            status: &data_status,
            description: &data_description,
            duration: data_duration,
            due_date: data_due_date,
            task_list: data_task_list.unwrap_or_default(),
        };

        let result = diesel::insert_into(tasks::table).values(&new_task).returning(Task::as_returning()).get_result(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_task(&self, task_payload: &TaskUpdatePayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let task_id_uuid = Uuid::parse_str(&task_payload.task_id).unwrap();
        let user_id_uuid = Uuid::parse_str(&task_payload.user_id.clone().unwrap_or_default()).ok();
        let target = tasks::table
            .filter(id.eq(task_id_uuid))
            .filter(user_id.eq(user_id_uuid.unwrap()));
        let data_task_list: Option<Vec<&str>> = task_payload.task_list.as_ref().map(|vec| vec.iter().map(|s| s.as_str()).collect());

        let result = diesel::update(target)
            .set((
                user_id_uuid.clone().map(|data| user_id.eq(data)),
                task_payload.title.clone().map(|data| title.eq(data)),
                task_payload.typ.clone().map(|data| typ.eq(data.to_string())),
                task_payload.priority.clone().map(|data| priority.eq(data.to_string())),
                task_payload.status.clone().map(|data| priority.eq(data.to_string())),
                task_payload.description.clone().map(|data| description.eq(data)),
                task_payload.duration.map(|data| duration.eq(data)),
                task_payload.due_date.map(|data| due_date.eq(data)),
                data_task_list.map(|data| task_list.eq(data)),
            ))
            .returning(Task::as_returning())
            .get_result(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_tasks(&self, task_payload: &TaskDataPayload) -> Result<Vec<TaskAllEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let user_id_uuid = task_payload.user_id.as_ref().and_then(|data: &String| Uuid::parse_str(data).ok());
        let mut query = tasks.into_boxed();
        if let Some(data) = user_id_uuid {
            query = query.filter(user_id.eq(data));
        }
        let results = query.load::<Task>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(TaskAllDbMapper::to_entity).collect::<Vec<TaskAllEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_task_by_id(&self, task_payload: &TaskDataPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let task_id_uuid = task_payload.task_id.as_ref().and_then(|data: &String| Uuid::parse_str(data).ok());
        let user_id_uuid = task_payload.user_id.as_ref().and_then(|data: &String| Uuid::parse_str(data).ok());

        let mut query = tasks.into_boxed();
        if let Some(data) = task_id_uuid {
            query = query.filter(id.eq(data));
        }
        if let Some(data) = user_id_uuid {
            query = query.filter(user_id.eq(data));
        }
        let result = query.get_result::<Task>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_task_by_id(&self, task_payload: &TaskDataPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let task_id_uuid = Uuid::parse_str(&task_payload.task_id.clone().unwrap()).unwrap();
        let result = tasks.filter(id.eq(task_id_uuid)).get_result::<Task>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
