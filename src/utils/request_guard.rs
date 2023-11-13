use jsonwebtoken::errors::Error;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use crate::models::jwt_model::{Claims, JWT};
use crate::models::response_model::{NetworkResponse, Response, ResponseBody};
use crate::utils::jwt_utils::decode_jwt;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }
        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    body: ResponseBody::Message(
                        String::from("Error validating JWT token - No token provided")
                    )
                };

                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                ))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Expired Token")
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Invalid Token")
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    },
                    _ => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - {}", err)
                            )
                        };

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    }
                }
            },
        }
    }
}