use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> BaseResponse<T> {
    pub fn new(code: u16, message: &str, data: T) -> Self {
        BaseResponse {
            code,
            message: message.to_string(),
            data,
        }
    }

    pub fn to_http_response(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
