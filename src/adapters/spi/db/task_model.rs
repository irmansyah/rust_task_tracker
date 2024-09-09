use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Selectable, AsChangeset, QueryableByName)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: Uuid,
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

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct TaskNew<'a> {
    pub title: &'a str,
    pub typ: &'a str,
    pub priority: &'a str,
    pub status: &'a str,
    pub description: &'a str,
    pub duration: i32,
    pub due_date: i64,
    pub project_id: i32,
    pub task_list: Vec<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct TaskUpdate<'a> {
    pub title: &'a str,
    pub typ: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub status: Option<&'a str>,
    pub description: &'a str,
    pub duration: Option<&'a i32>,
    pub due_date: Option<&'a i64>,
    pub project_id: Option<&'a i32>,
    pub task_list: Option<Vec<&'a str>>,
    pub updated_at: NaiveDateTime,
}
