use diesel::prelude::*;
use serde::Deserialize;
use tasktracker_backend::adapters::spi::db::schema::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Deserialize, Queryable, Insertable, Selectable, AsChangeset, QueryableByName)]
#[diesel(table_name = tasks)]
pub struct TaskJson {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub typ: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub description: String,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
