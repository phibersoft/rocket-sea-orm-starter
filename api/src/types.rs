use std::io::Cursor;

use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: usize,
}

impl ErrorResponse {
    pub fn new(message: Option<&'static str>, status: Option<usize>) -> Self {
        Self {
            message: message.unwrap_or("Unknown error occured.").to_string(),
            status: status.unwrap_or(500),
        }
    }

    // Custom Errors
    #[allow(unused)]
    pub fn unauthorized() -> Self {
        Self::new(Some("Unauthorized Access"), Some(401))
    }
}

impl From<rocket::serde::json::Error<'_>> for ErrorResponse {
    fn from(err: rocket::serde::json::Error<'_>) -> Self {
        Self {
            message: err.to_string(),
            status: 422,
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json_string = match serde::serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => return Err(Status::InternalServerError),
        };

        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(Status::new(self.status as u16))
            .sized_body(json_string.len(), Cursor::new(json_string))
            .ok()
    }
}

#[macro_export]
macro_rules! throw {
    ($message:expr, $status:expr) => {
        return Err($crate::ErrorResponse::new(Some($message), Some($status)));
    };
}

#[allow(unused)]
pub type JsonResponse<T> = Result<Json<T>, ErrorResponse>;

#[allow(unused)]
pub type JsonErrorResult<'a, T> = Result<Json<T>, rocket::serde::json::Error<'a>>;