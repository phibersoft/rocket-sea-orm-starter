use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;

use db::tables::user::UserResponse;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::error::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub name: String,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    #[allow(unused)]
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Self::check_jwt(request) {
            Ok(jwt) => Outcome::Success(jwt),
            Err(error_response) => error_response.as_outcome::<Self>()
        }
    }
}

impl JWT {
    fn secret() -> String {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set.")
    }

    pub fn create_jwt(user: UserResponse) -> Result<String, jsonwebtoken::errors::Error> {
        let seconds: u64 = env::var("JWT_EXPIRES").expect("JWT_EXPIRES must be set.").parse().expect("JWT_EXPIRES must be parseable.");
        let expiration = SystemTime::now().checked_add(Duration::from_secs(seconds)).expect("Invalid Timestamp").duration_since(UNIX_EPOCH).expect("Time went backwards.").as_secs();

        let claims = Claims {
            id: user.id,
            name: user.name,
            email: user.email,
            exp: expiration as usize,
        };

        let header = Header::new(Algorithm::HS512);

        encode(&header, &claims, &EncodingKey::from_secret(Self::secret().as_bytes()))
    }

    pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
        let token = token.trim_start_matches("Bearer").trim();

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(Self::secret().as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(err.kind().to_owned())
        }
    }

    pub fn check_jwt(request: &Request) -> Result<JWT, ErrorResponse> {
        fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
            Ok(JWT::decode_jwt(key.to_string())?)
        }

        match request.headers().get_one("authorization") {
            None => Err(ErrorResponse::unauthorized()),
            Some(key) => match is_valid(key) {
                Ok(claims) => Ok(JWT { claims }),
                Err(err) => match err {
                    ErrorKind::ExpiredSignature => Err(ErrorResponse::from(("Expired Signature. Login again.", 401))),
                    ErrorKind::InvalidToken => Err(ErrorResponse::from(("Invalid Token.", 401))),
                    _ => Err(ErrorResponse::from(("JWT Decode Error. (unhandled)", 401)))
                }
            },
        }
    }
}

