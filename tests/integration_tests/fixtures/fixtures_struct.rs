// use tasktracker_backend::adapters::spi::db::schema::*;
// use diesel::Insertable;
// use serde::Deserialize;

// #[derive(Deserialize, Insertable, Debug)]
// #[table_name = "tasks"]
// pub struct TaskJson {
//     pub id: i32,
//     pub title: String,
// }

use diesel::prelude::*;
use tasktracker_backend::adapters::spi::db::schema::tasks;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Selectable, AsChangeset, Debug)]
#[table_name = "tasks"]
pub struct TaskJson {
    pub id: Uuid,
    pub title: String,
    pub typ: String,
    pub priority: String,
    pub status: String,
    pub description: String,
    pub duration: i32,
    pub due_date: i64,
    pub project_id: i32,
    pub task_list: Vec<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
