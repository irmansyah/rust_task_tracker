use actix_web::HttpResponse;
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(code: StatusCode, message: &str, data: T) -> Self {
        SuccessResponse {
            code: code.as_u16(),
            message: message.to_string(),
            data,
        }
    }

    pub fn to_http_response(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
