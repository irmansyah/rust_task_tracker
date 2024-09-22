use crate::domain::error::ApiError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub message: String,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", error)]
pub struct ErrorResponse {
    code: StatusCode,
    error: String,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        ErrorResponse {
            code: StatusCode::UNAUTHORIZED,
            error: "Permission denied".to_string(),
        }
    }
}


impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let error_response = ErrorPresenter {
            code: code.as_u16(),
            message: self.error.clone(),
        };
        HttpResponse::build(code).json(error_response)
    }
}

impl ErrorResponse {
    pub fn map_io_error(e: ApiError) -> ErrorResponse {
        match e.get_error_code() {
            400 => ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                error: e.get_error_message(),
            },
            401 => ErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                error: e.get_error_message(),
            },
            403 => ErrorResponse {
                code: StatusCode::FORBIDDEN,
                error: e.get_error_message(),
            },
            404 => ErrorResponse {
                code: StatusCode::NOT_FOUND,
                error: e.get_error_message(),
            },
            _ => ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                error: String::from("Error: an unknown error occured"),
            },
        }
    }
}
