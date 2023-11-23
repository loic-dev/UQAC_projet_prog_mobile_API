use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, Request, serde::json::Json, State};
use crate::models::jwt_model::JWT;
use crate::models::status_model::{FailureResponse, SuccessResponse};

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String, _key: JWT) -> Result<Json<SuccessResponse<User>>, Json<FailureResponse>> {
    let id = path;
    if id.is_empty() {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: "Id is null".to_string()
        }))
    };
    let user_detail = db.get_user(&id);
    return match user_detail {
        Ok(user) =>
            Ok(Json::from(SuccessResponse {
                code: Status::Accepted,
                message: user
            })),
        Err(_) =>
            Err(Json::from(FailureResponse {
                code: Status::InternalServerError,
                error: "Internal Server Error".to_string()
            })),
    }
}

#[delete("/user/<path>")]
pub fn delete_user(db: &State<MongoRepo>, path: String, _key: JWT) -> Result<Json<SuccessResponse<String>>, Json<FailureResponse>> {
    let id = path;
    if id.is_empty() {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: "Id is null".to_string()
        }))
    };
    let result = db.delete_user(&id);
    return match result {
        Ok(_) =>
            Ok(Json::from(SuccessResponse {
                code: Status::Accepted,
                message: "User successfully deleted".to_string()
            })),
        Err(_) =>
            Err(Json::from(FailureResponse {
                code: Status::InternalServerError,
                error: "Internal Server Error while deleting user".to_string()
            })),
    }
}

#[get("/api")]
pub fn auth(_key: JWT) -> Result<Json<SuccessResponse<String>>,()>{
    Ok(Json::from(SuccessResponse {
        code: Status::Accepted,
        message: "JWT token is valid".to_string()
    }))
}

#[catch[401]]
pub fn jwt_unauthorized(status: Status, _req: &Request) -> Result<(),Json<FailureResponse>>{
    Err(Json::from(FailureResponse {
        code: status,
        error: "Unauthorized JWT".to_string()
    }))
}