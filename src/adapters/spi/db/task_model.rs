use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub typ: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub description: String,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
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

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = tasks)]
// pub struct TaskAll {
//     pub id: i32,
//     pub title: String,
//     pub description: String,
// }


// #[derive(Insertable, AsChangeset)]
// #[diesel(table_name = tasks)]
// pub struct TaskAllNew<'a> {
//     pub title: &'a str,
//     pub description: &'a str,
// }
