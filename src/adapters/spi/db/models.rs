use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName, Insertable)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub task: String,
}

impl Task {
    pub fn new(id: i32, task: String) -> Self {
        Task { id, task }
    }
}
