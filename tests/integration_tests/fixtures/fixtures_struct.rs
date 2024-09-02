use tasktracker_backend::adapters::spi::db::schema::*;
use diesel::Insertable;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "tasks"]
pub struct TaskJson {
    pub id: i32,
    pub task: String,
}
