use rocket::http::Status;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStatus {
    pub code: Status,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedStatus {
    pub code: Status,
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqwestStatus {
    pub code: Status,
    pub message: Value,
}