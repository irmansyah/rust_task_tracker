use crate::adapters::spi::db::db_tasks_repository::TasksRepository;

pub struct AppState {
    pub app_name: String,
    pub tasks_repository: TasksRepository,
}
