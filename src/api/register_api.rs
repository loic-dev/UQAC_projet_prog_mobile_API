use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, utils::data_check};
use pwhash::bcrypt::hash;
use rocket::{http::Status, serde::json::Json, State};
use regex::Regex;
use crate::models::status_model::{FailureResponse, SuccessResponse};

#[post("/register", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<SuccessResponse<String>>, Json<FailureResponse>> {
    let re_email = Regex::new(r"^[A-Za-z0-9.?]+@[A-Za-z]+\.[A-Za-z]{2,3}$").unwrap();
    let check_pass = data_check::is_valid_password(&*new_user.password);
    let email_exists = data_check::email_exist(db, &*new_user.email);

    if email_exists {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: ("Email already exists").to_string(),
        })
        )
    }

    if !re_email.is_match(&*new_user.email) {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: ("Bad Regex for e-mail").to_string(),
        })
        )
    }

    if !check_pass {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: ("Bad Regex for password").to_string(),
        })
        )
    }

    let hash_pass = hash(&*new_user.password).unwrap();
    let data = User {
        id: None,
        firstname: new_user.firstname.to_owned(),
        lastname: new_user.lastname.to_owned(),
        email: new_user.email.to_owned(),
        password: hash_pass.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(_) => Ok(Json::from(SuccessResponse {
            code: Status::Ok,
            message: ("User has been created successfully").to_string(),
        })),
        Err(_) => Err(Json::from(FailureResponse {
            code: Status::InternalServerError,
            error: ("Internal Server Error").to_string(),
        }))
    }
}