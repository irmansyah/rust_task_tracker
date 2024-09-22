use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    adapters::api::{tasks::tasks_controllers, users::users_controllers},
    // application::utils::auth::jwt::validator,
};

pub fn routes(config: &mut web::ServiceConfig) {
    // let bearer_middleware = HttpAuthentication::bearer(validator);
    config
        // .service(web::scope("/api/v1/users").wrap(bearer_middleware.clone()).configure(users_controllers::routes))
        .service(web::scope("/api/v1/users").configure(users_controllers::routes))
        .service(web::scope("/api/v1/tasks").configure(tasks_controllers::routes));
}

