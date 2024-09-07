use crate::adapters::spi::db::{db_tasks_repository::TasksRepository, db_users_repository::UsersRepository};

pub struct AppState {
    pub app_name: String,
    pub users_repository: UsersRepository,
    pub tasks_repository: TasksRepository,
}
