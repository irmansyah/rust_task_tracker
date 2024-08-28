use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::RunQueryDsl;

use super::models::Task;
use super::schema::tasks;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_name: String,
}

impl DbConnection {
    pub fn get_pool(&self) -> DbPool {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = format!("{}/{}", database_url, &self.db_name);

        let manager = ConnectionManager::<PgConnection>::new(&database);

        r2d2::Pool::new(manager).unwrap()
    }

    pub fn create_task(&self, conn: &mut PgConnection, new_task: &Task) -> Result<Task, diesel::result::Error> {
        diesel::insert_into(tasks::table)
        .values(new_task)
        .get_result(conn)
    }
}

    // pub fn get_pool(&self) -> DbPool {
    //     let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //     let database = format!("{}/{}", database_url, &self.db_name);

    //     let manager = ConnectionManager::<PgConnection>::new(&database);
    //     let pool = r2d2::Pool::new(manager).expect("Failed to create pool");

    //     // Ensure the `dog_facts` table exists
    //     let conn = pool.get().expect("Failed to get a database connection");
    //     self.ensure_table_exists(&conn, "dog_facts");

    //     pool
    // }

    // fn ensure_table_exists(&self, conn: &PooledConnection<ConnectionManager<PgConnection>>, table_name: &str) {
    //     if !self.table_exists(conn, table_name) {
    //         self.create_table(conn);
    //     }
    // }

    // fn table_exists(&self, conn: &PooledConnection<ConnectionManager<PgConnection>>, table_name: &str) -> bool {
    //     let query = format!("SELECT to_regclass('{}') IS NOT NULL as exists", table_name);
    //     let result: bool = sql_query(query).get_result(&conn).expect("Failed to execute check table query");

    //     result
    // }

    // fn create_table(&self, conn: &PooledConnection<ConnectionManager<PgConnection>>) {
    //     let create_table_query = "
    //         CREATE TABLE dog_facts (
    //             id SERIAL PRIMARY KEY,
    //             fact TEXT NOT NULL,
    //             source VARCHAR NOT NULL
    //         );
    //     ";

    //     sql_query(create_table_query).execute(&conn).expect("Failed to create dog_facts table");
    // }
// }
