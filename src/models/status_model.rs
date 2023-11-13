use rocket::http::Status;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStatus {
    pub code: Status,
    pub message: String,
}