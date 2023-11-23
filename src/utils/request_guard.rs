use jsonwebtoken::errors::Error;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use crate::models::jwt_model::{Claims, JWT};
use crate::models::status_model::FailureResponse;
use crate::utils::jwt_utils::decode_jwt;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = FailureResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, FailureResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }
        match req.headers().get_one("authorization") {
            None => {
                Outcome::Error((
                    Status::Unauthorized,
                    FailureResponse {
                        code: Status::Unauthorized,
                        error: "No JWT token provided".to_string()
                    }
                ))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Outcome::Error((
                            Status::Unauthorized,
                            FailureResponse {
                                code: Status::Unauthorized,
                                error: "JWT token has expired ".to_string()
                            }
                        ))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        Outcome::Error((
                            Status::Unauthorized,
                            FailureResponse {
                                code: Status::Unauthorized,
                                error: "Invalid JWT token".to_string()
                            }
                        ))
                    },
                    _ => {
                        Outcome::Error((
                            Status::Unauthorized,
                            FailureResponse {
                                code: Status::Unauthorized,
                                error: format!("Error validating JWT Token : {}", err).to_string()
                            }
                        ))
                    }
                }
            },
        }
    }
}