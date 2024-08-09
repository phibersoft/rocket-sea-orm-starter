use std::env;

use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;

use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::types::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
            Ok(decode_jwt(key.to_string())?)
        }

        match request.headers().get_one("authorization") {
            None => {
                Outcome::Error((Default::default(), ErrorResponse::unauthorized()))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => {
                    Outcome::Success(JWT { claims })
                }
                Err(err) => match err {
                    ErrorKind::ExpiredSignature => Outcome::Error((Status::new(402), ErrorResponse::new(Some("Expired signature"), None))),
                    ErrorKind::InvalidToken => Outcome::Error((Status::new(401), ErrorResponse::new(Some("Invalid Token"), None))),
                    _ => Outcome::Error((Status::new(401), ErrorResponse::new(Some("Other type of errors"), None)))
                }
            },
        }
    }
}

pub fn create_jwt(id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}