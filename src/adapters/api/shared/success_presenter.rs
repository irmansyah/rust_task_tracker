// use actix_web::{http::StatusCode, HttpResponse};
// use derive_more::Display;
// use serde::Deserialize;
// use serde::Serialize;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SuccessPresenter {
//     pub code: u16,
//     pub message: String,
// }

// #[derive(Debug, Display)]
// pub struct SuccessResponse {
//     code: StatusCode,
//     error: String,
// }

// // impl ResponseSuccess for SuccessResponse {
// //     fn status_code(&self) -> StatusCode {
// //         self.code
// //     }

// //     fn error_response(&self) -> HttpResponse {
// //         let code = self.status_code();
// //         let error_response = SuccessPresenter {
// //             code: code.as_u16(),
// //             message: self.error.clone(),
// //         };
// //         HttpResponse::build(code).json(error_response)
// //     }
// // }

// impl SuccessResponse {
//     pub fn map_io_error(e: ApiSuccess) -> SuccessResponse {
//         match e.get_error_code() {
//             200 => SuccessResponse {
//                 code: StatusCode::OK,
//                 error: e.get_error_message(),
//             },
//             401 => SuccessResponse {
//                 code: StatusCode::CREATED,
//                 error: e.get_error_message(),
//             },
//             _ => SuccessResponse {
//                 code: StatusCode::INTERNAL_SERVER_ERROR,
//                 error: String::from("Success: an unknown error occured"),
//             },
//         }
//     }
// }
