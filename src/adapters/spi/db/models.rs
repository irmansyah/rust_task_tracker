use crate::adapters::spi::db::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub title: Option<String>,
    pub typ: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<String>>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: Option<&'a str>,
    pub typ: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub status: Option<&'a str>,
    pub description: Option<&'a str>,
    pub duration: Option<i32>,
    pub due_date: Option<i64>,
    pub project_id: Option<i32>,
    pub task_list: Option<Vec<&'a str>>,
}

// #[derive(Queryable, Identifiable, Insertable, Debug)]
// #[diesel(table_name = tasks)]
// pub struct Task {
//     pub id: i32,
//     pub title: String,
//     pub typ: Option<String>,
//     pub priority: Option<String>,
//     pub status: Option<String>,
//     pub description: Option<String>,
//     pub duration: Option<i32>,
//     pub due_date: Option<i64>,
//     pub project_id: Option<i32>,
//     pub task_list: Vec<String>,
// }

// impl Task {
//     pub fn new(
//         id: i32,
//         title: String,
//         typ: Option<String>,
//         priority: Option<String>,
//         status: Option<String>,
//         description: Option<String>,
//         duration: Option<i32>,
//         due_date: Option<i64>,
//         project_id: Option<i32>,
//         task_list: Vec<String>,
//     ) -> Self {
//         Task {
//             id,
//             title,
//             typ,
//             priority,
//             status,
//             description,
//             duration,
//             due_date,
//             project_id,
//             task_list,
//         }
//     }
// }

// #[derive(Insertable)]
// #[diesel(table_name = tasks)]
// pub struct NewTask<'a> {
//     pub id: Uuid,
//     pub title: &'a str,
//     pub typ: Option<&'a str>,
//     pub priority: Option<&'a str>,
//     pub status: Option<&'a str>,
//     pub description: Option<&'a str>,
//     pub duration: Option<i32>,
//     pub due_date: Option<i64>,
//     pub project_id: Option<&'a str>,
//     pub task_list: Vec<&'a str>,
// }

// #[derive(AsChangeset)]
// #[diesel(table_name = tasks)]
// pub struct UpdateTask<'a> {
//     pub title: Option<&'a str>,
//     pub typ: Option<&'a str>,
//     pub priority: Option<&'a str>,
//     pub status: Option<&'a str>,
//     pub description: Option<&'a str>,
//     pub duration: Option<i32>,
//     pub due_date: Option<i64>,
//     pub project_id: Option<&'a str>,
//     pub task_list: Vec<&'a str>,
// }
