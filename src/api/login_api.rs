use crate::repository::mongodb_repo::MongoRepo;
use pwhash::bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};
use crate::models::status_model::{LoginResponse, FailureResponse, SuccessResponse};
use crate::models::user_model::LoginRequest;
use crate::utils::jwt_utils::create_jwt;

#[post("/login", format="application/json", data="<user>")]
pub fn login_user(db: &State<MongoRepo>,
                  user: Json<LoginRequest>) -> Result<Json<SuccessResponse<LoginResponse>>, Json<FailureResponse>> {
    let login_data = LoginRequest {
        email: user.email.to_owned(),
        password: user.password.to_owned()
    };
    let user_detail = db.get_user_by_email(&*user.email);
    if user_detail.is_none() {
        return Err(Json::from(FailureResponse {
            code: Status::NotFound,
            error: ("User not found").to_string(),
        }))
    }
    let user = user_detail.unwrap();
    let pass_valid = verify(login_data.password, &*user.password);
    return if login_data.email == user.email && pass_valid {
        let token = create_jwt(user.id);
        match token {
            Ok(token) => {
                let login = LoginResponse { user, token };
                Ok(Json::from(SuccessResponse {
                    code: Status::Ok,
                    message: login,
                }))
            }
            Err(_) => {
                Err(Json::from(FailureResponse {
                    code: Status::InternalServerError,
                    error: "Internal Server Error".to_string(),
                }))
            }
        }
    } else {
        Err(Json::from(FailureResponse {
            code: Status::Unauthorized,
            error: ("Bad credentials").to_string(),
        }))
    }
}
