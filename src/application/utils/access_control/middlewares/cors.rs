use actix_web::http::{header, Method};
use actix_cors::Cors;

pub fn cors(client_origin_url: &str) -> Cors {
    Cors::default()
        .allowed_origin(client_origin_url)
        .allowed_methods([Method::GET])
        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(86_400)
}
