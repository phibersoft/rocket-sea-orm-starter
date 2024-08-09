use std::io::{Cursor, Error};

use db::sea_orm::{DbErr, RuntimeErr, SqlxError};
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: usize,
}

impl ErrorResponse {
    pub fn new(message: Option<String>, status: Option<usize>) -> Self {
        Self {
            message: message.unwrap_or(String::from("Unknown error occured.")),
            status: status.unwrap_or(500),
        }
    }

    // Custom Errors
    #[allow(unused)]
    pub fn unauthorized() -> Self {
        Self::from(("Unauthorized Access.", 401))
    }

    #[allow(unused)]
    pub fn database(db_err: DbErr) -> Self {
        match db_err {
            DbErr::Query(RuntimeErr::SqlxError(SqlxError::Database(ref db_error))) => {
                match db_error.code() {
                    Some(code) => {
                        let code_string = code.to_string();
                        let code = code_string.as_str();

                        match code {
                            "23505" => Self::from((format!("{}.{} already exists.", db_error.table().unwrap_or(""), db_error.constraint().unwrap_or("")), 400)),
                            _ => Self::from((db_error.message(), 400))
                        }
                    }
                    None => Self::from((db_error.message(), 500))
                }
            }
            e => {
                let error_message = "Database Error: ".to_string() + &e.to_string();
                Self::from((error_message, 500))
            }
        }
    }

    #[allow(unused)]
    pub fn not_found() -> Self { Self::from(("Not found.", 404)) }

    #[allow(unused)]
    pub fn as_outcome<SuccessResponse>(&self) -> Outcome<SuccessResponse, ErrorResponse> {
        Outcome::Error((Default::default(), ErrorResponse::from(self)))
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            message: String::from("Unknown error occured."),
            status: 500,
        }
    }
}

impl From<String> for ErrorResponse {
    fn from(message: String) -> Self { ErrorResponse::new(Some(message), None) }
}

impl From<&str> for ErrorResponse {
    fn from(message: &str) -> Self { ErrorResponse::new(Some(message.to_string()), None) }
}

impl From<(&str, usize)> for ErrorResponse {
    fn from(value: (&str, usize)) -> Self { ErrorResponse::new(Some(value.0.to_string()), Some(value.1)) }
}

impl From<(String, usize)> for ErrorResponse {
    fn from(value: (String, usize)) -> Self { ErrorResponse::new(Some(value.0), Some(value.1)) }
}

impl From<&ErrorResponse> for ErrorResponse {
    fn from(value: &ErrorResponse) -> Self {
        Self::from((value.message.clone(), value.status))
    }
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        Self::from(value.to_string())
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
        return Err($crate::ErrorResponse::from(($message, $status)));
    };
}