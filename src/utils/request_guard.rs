use jsonwebtoken::errors::Error;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use crate::models::jwt_model::{Claims, JWT};
use crate::models::status_model::CustomStatus;
use crate::utils::jwt_utils::decode_jwt;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = CustomStatus;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, CustomStatus> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }
        match req.headers().get_one("authorization") {
            None => {
                Outcome::Error((
                    Status::Unauthorized,
                    CustomStatus {
                        code: Status::Unauthorized,
                        message: "No JWT token provided".to_string()
                    }
                ))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Outcome::Error((
                            Status::Unauthorized,
                            CustomStatus {
                                code: Status::Unauthorized,
                                message: "JWT token has expired ".to_string()
                            }
                        ))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        Outcome::Error((
                            Status::Unauthorized,
                            CustomStatus {
                                code: Status::Unauthorized,
                                message: "Invalid JWT token".to_string()
                            }
                        ))
                    },
                    _ => {
                        Outcome::Error((
                            Status::Unauthorized,
                            CustomStatus {
                                code: Status::Unauthorized,
                                message: format!("Error validating JWT Token : {}", err).to_string()
                            }
                        ))
                    }
                }
            },
        }
    }
}