use tasktracker_backend::adapters::spi::db::db_connection::DbConnection;
use tasktracker_backend::adapters::spi::db::schema::tasks::dsl::*;
use diesel::{insert_into, RunQueryDsl};

use crate::{integration_tests::fixtures::fixtures_struct::TaskJson, utils::utils_file::read_from_file};

pub fn execute_imports(conn: &DbConnection) {
    import_task_fixtures(conn);
}

fn import_task_fixtures(conn: &DbConnection) {
    let json = read_from_file::<Vec<TaskJson>>("tests/integration_tests/fixtures/tasks.json").unwrap();

    let mut conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(tasks).values(&json).execute(&mut conn).unwrap();
}
