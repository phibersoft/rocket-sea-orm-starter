use std::env;

use cookie::time::{Duration, OffsetDateTime};

use db::{Connection, pool};
use db::sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait};
use db::sea_orm::query::*;
use db::tables::{user, User};
use rocket::{post, Route, routes};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::authentication::JWT;
use crate::error::ErrorResponse;
use crate::types::JsonResponse;

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub struct AuthenticationResponse {
    user: user::UserResponse,
    token: String,
}

fn common_response(user: user::Model, cookie_jar: &CookieJar<'_>) -> JsonResponse<AuthenticationResponse> {
    let user_response: user::UserResponse = user.into();
    let token = JWT::create_jwt(user_response.clone());

    if token.is_err() {
        return Err(ErrorResponse::from("Cookie generation."));
    }

    let token = token.unwrap();
    let jwt_expires: i64 = env::var("JWT_EXPIRES").unwrap().parse().expect("JWT_EXPIRES should be parseable.");
    let expiration_time = OffsetDateTime::now_utc() + Duration::seconds(jwt_expires);

    let cookie = Cookie::build(("auth", token.clone())).expires(expiration_time);

    cookie_jar.add(cookie);

    Ok(Json(AuthenticationResponse {
        user: user_response,
        token,
    }))
}

#[post("/register", format = "json", data = "<input_data>")]
async fn register(cookie_jar: &CookieJar<'_>, conn: Connection<pool::Db>, input_data: Json<user::RegisterInput>) -> JsonResponse<AuthenticationResponse> {
    let data = input_data.into_inner();

    let user = user::Model::new(data);
    let response: Result<user::Model, DbErr> = user.insert(&conn.into_inner()).await;

    match response {
        Ok(user) => common_response(user, cookie_jar),
        Err(e) => Err(ErrorResponse::database(e))
    }
}

#[post("/login", format = "json", data = "<input_data>")]
async fn login(cookie_jar: &CookieJar<'_>, conn: Connection<pool::Db>, input_data: Json<user::LoginInput>) -> JsonResponse<AuthenticationResponse> {
    let data = input_data.into_inner();

    let response = User::find()
        .filter(user::Column::Email.eq(data.email))
        .one(&conn.into_inner())
        .await;

    match response {
        Ok(user) => {
            match user {
                Some(user) => common_response(user, cookie_jar),
                None => Err(ErrorResponse::not_found())
            }
        }
        Err(e) => Err(ErrorResponse::database(e))
    }
}


pub fn routes() -> Vec<Route> {
    routes![login, register]
}