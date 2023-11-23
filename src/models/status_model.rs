use rocket::http::Status;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::user_model::User;


#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub code: Status,
    pub message: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FailureResponse {
    pub code: Status,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqwestStatus {
    pub code: Status,
    pub message: Value,
}