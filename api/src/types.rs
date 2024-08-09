use rocket::serde::json::Json;

use crate::authentication::JWT;
use crate::error::ErrorResponse;

#[allow(unused)]
pub type JsonResponse<T> = Result<Json<T>, ErrorResponse>;

#[allow(unused)]
pub type JsonErrorResult<'a, T> = Result<Json<T>, rocket::serde::json::Error<'a>>;

#[allow(unused)]
pub type Authenticated = Result<JWT, ErrorResponse>;