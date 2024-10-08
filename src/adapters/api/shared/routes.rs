use actix_web::web;

use crate::adapters::api::{tasks::tasks_controllers, users::users_controllers};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/users").configure(users_controllers::routes))
        .service(web::scope("/api/v1/tasks").configure(tasks_controllers::routes));
}
