use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

use crate::adapters::api::tasks::tasks_payloads::TaskPayload;
use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::TaskDbMapper, models::Task, schema::tasks::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::tasks_repository_abstract::TasksRepositoryAbstract};
use crate::domain::task_entity::TaskEntity;

use crate::adapters::spi::db::schema::tasks;
use crate::adapters::spi::db::schema::tasks::id;

use super::models::{NewTask, UpdateTask};

pub struct TasksRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl TasksRepositoryAbstract for TasksRepository {
    async fn post_one_task(&self, task_payload: &TaskPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let new_task = NewTask {
            title: &task_payload.title,
            typ: Some(&task_payload.typ.to_string()),
            priority: Some(&task_payload.priority.to_string()),
            status: Some(&task_payload.status.to_string()),
            description: task_payload.description.as_deref(),
            duration: task_payload.duration,
            due_date: task_payload.due_date,
            project_id: task_payload.project_id.as_deref(),
            task_list: Some(task_payload.task_list.map(|f| f.to_string())),
        };

        let result = diesel::insert_into(tasks::table).values(TaskDbMapper::to_db(new_task)).get_result::<NewTask>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_one_task(&self, task_payload: &TaskPayload) -> Result<TaskEntity, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let mut filter = tasks::table.filter(tasks::id.eq(task_payload.task_id));

        let update_task = UpdateTask {
            title: task_payload.title.to_str(),
            typ: task_payload.typ.to_string().clone(),
            priority: task_payload.priority.to_string().clone(),
            status: task_payload.status.to_string().clone(),
            description: task_payload.description.clone(),
            duration: task_payload.duration,
            due_date: task_payload.due_date,
            project_id: task_payload.project_id,
            task_list: task_payload.task_list,
        };
        let result = diesel::update(filter).set(update_task).get_result::<Task>(&mut conn);

        // let result = diesel::update(filter)
        //     .set((
        //             tasks::title.eq(task_payload.title),
        //             tasks::typ.eq(task_payload.typ.to_string()),
        //             tasks::priority.eq(task_payload.priority.to_string()),
        //             tasks::status.eq(task_payload.status.to_string()),
        //             tasks::description.eq(Some(task_payload.description)),
        //             tasks::duration.eq(Some(task_payload.duration)),
        //             tasks::due_date.eq(Some(task_payload.due_date_int)),
        //             tasks::project_id.eq(Some(task_payload.project_id)),
        //             tasks::task_list.eq(task_payload.task_list),
        //     ))
        //     .get_result::<Task>(&mut conn);

        // title -> Varchar,
        // typ -> Varchar,
        // priority -> Varchar,
        // status -> Varchar,
        // description -> Nullable<Varchar>,
        // duration -> Nullable<Int4>,
        // due_date -> Nullable<BigInt>,
        // project_id -> Nullable<Int4>,
        // task_list ->  Nullable<Array<Text>>,

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
