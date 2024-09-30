use crate::application::utils::access_control::extractors::claims::ClientError;
use crate::domain::error::ApiError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "{:?}", message)]
pub struct ErrorResponse {
    code: StatusCode,
    message: String,
    data: Option<serde_json::Value>,
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let error_response = ErrorPresenter {
            code: code.as_u16(),
            message: self.message.clone(),
            data: None,
        };
        HttpResponse::build(code).json(error_response)
    }
}

impl ErrorResponse {
    pub fn map_io_error(e: ApiError) -> ErrorResponse {
        match e.get_error_code() {
            400 => ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: e.get_error_message(),
                data: None,
            },
            401 => ErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: e.get_error_message(),
                data: None,
            },
            403 => ErrorResponse {
                code: StatusCode::FORBIDDEN,
                message: e.get_error_message(),
                data: None,
            },
            404 => ErrorResponse {
                code: StatusCode::NOT_FOUND,
                message: e.get_error_message(),
                data: None,
            },
            _ => ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error: an unknown error occured"),
                data: None,
            },
        }
    }

    pub fn map_io_error_default(e: String) -> ErrorResponse {
        ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: e,
            data: None,
        }
    }

    pub fn auth_default() -> ErrorResponse {
        ErrorResponse {
            code: StatusCode::UNAUTHORIZED,
            message: "Permission denied".to_string(),
            data: None,
        }
    }
}

impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                code: Some(StatusCode::UNAUTHORIZED.as_u16()),
                message: Some("Requires authentication".to_string()),
                data: None,
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                code: Some(StatusCode::UNAUTHORIZED.as_u16()),
                message: Some("Bad credentials".to_string()),
                data: None,
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                code: Some(StatusCode::UNAUTHORIZED.as_u16()),
                message: Some(msg.to_string()),
                data: None,
            }),
            Self::UnsupportedAlgortithm(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                code: Some(StatusCode::UNAUTHORIZED.as_u16()),
                message: Some("Bad credentials".to_string()),
                data: None,
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
