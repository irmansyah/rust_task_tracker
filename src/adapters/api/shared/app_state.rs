use crate::adapters::spi::{
    db::{db_dog_facts_repository::DogFactsRepository, db_tasks_repository::TasksRepository},
    http::http_cat_facts_repository::CatFactsRepository,
};

pub struct AppState {
    pub app_name: String,
    pub cats_repository: CatFactsRepository,
    pub dogs_repository: DogFactsRepository,
    pub tasks_repository: TasksRepository,
}
