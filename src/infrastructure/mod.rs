use std::{env, net::TcpListener, sync::Arc};

use crate::{
    adapters::{
        self,
        api::shared::app_state::AppState,
        spi::db::{db_connection::DbConnection, db_tasks_repository::TasksRepository, db_users_repository::UsersRepository},
    },
    application::utils::access_control::middlewares::{logger, security_headers},
};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub mod env_source;

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let _try_init = env_logger::try_init();

    let db_connection = Arc::new(DbConnection { db_name: db_name.to_string() });
    // let http_connection = HttpConnection {};

    let data = web::Data::new(AppState {
        app_name: String::from("Task Tracker API"),
        // cats_repository: CatFactsRepository {
        //     http_connection,
        //     source: dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
        // },
        users_repository: UsersRepository {
            db_connection: db_connection.clone(),
        },
        tasks_repository: TasksRepository {
            db_connection: db_connection.clone(),
        },
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(security_headers::security_headers())
            .wrap(logger::logger())
            .configure(adapters::api::shared::routes::routes)
    })
    .listen(listener)?
    .run();

    println!("Server running on port : {}, db_name : {}", port, db_name);

    Ok(server)
}
