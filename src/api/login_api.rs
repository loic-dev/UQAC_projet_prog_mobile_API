use crate::repository::mongodb_repo::MongoRepo;
use pwhash::bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};
use crate::models::status_model::{CustomStatus, LoggedStatus};
use crate::models::user_model::LoginRequest;
use crate::utils::jwt_utils::create_jwt;

#[post("/login", format="application/json", data="<user>")]
pub fn login_user(db: &State<MongoRepo>,
                  user: Json<LoginRequest>) -> Result<Json<LoggedStatus>, Json<CustomStatus>> {
    let login_data = LoginRequest {
        email: user.email.to_owned(),
        password: user.password.to_owned()
    };
    let user_detail = db.get_user_by_email(&*user.email).unwrap();
    let pass_valid = verify(login_data.password, &*user_detail.password);
    if login_data.email == user_detail.email && pass_valid {
        let token = create_jwt(user_detail.id);
        return Ok(Json::from(LoggedStatus {
            code: Status::Ok,
            user: user_detail,
            token: token.unwrap(),
        }))
    }
    else {
        return Err(Json::from(CustomStatus {
            code: Status::Unauthorized,
            message: ("Bad credentials").to_string(),
        })
        )
    }
}
