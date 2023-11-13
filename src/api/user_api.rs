use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, utils::data_check};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use pwhash::bcrypt::hash;
use pwhash::bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};
use regex::Regex;
use crate::models::jwt_model::JWT;
use crate::models::response_model::{NetworkResponse, Response, ResponseBody};
use crate::models::status_model::CustomStatus;
use crate::models::user_model::LoginRequest;
use crate::utils::jwt_utils::create_jwt;


#[post("/signup", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Json<CustomStatus>> {
    // Creating Regex to match email address
    let re_email = Regex::new(r"^[A-Za-z0-9.?]+@[A-Za-z]+\.[A-Za-z]{2,3}$").unwrap();
    let check_pass = data_check::is_valid_password(&*new_user.password);
    let email_exists = data_check::email_exist(db, &*new_user.email);

    if email_exists {
        println!("Email already taken");
        return Err(Json::from(CustomStatus {
            code: Status::BadRequest,
            message: ("Email already exists").to_string(),
        })
        )
    }

    if !re_email.is_match(&*new_user.email) {
        return Err(Json::from(CustomStatus {
            code: Status::BadRequest,
            message: ("Bad Regex for e-mail").to_string(),
        })
        )
    }

    if !check_pass {
        return Err(Json::from(CustomStatus {
            code: Status::BadRequest,
            message: ("Bad Regex for password").to_string(),
        })
        )
    }

    let hash_pass = hash(&*new_user.password).unwrap();
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: hash_pass.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Json::from(CustomStatus {
            code: Status::InternalServerError,
            message: ("Internal Server Error").to_string(),
        }),
        )
    }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<path>")]
pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", format="application/json", data="<user>")]
pub fn login_user(db: &State<MongoRepo>,
                  user: Json<LoginRequest>) -> Result<Json<CustomStatus>, Json<CustomStatus>> {
    let login_data = LoginRequest {
        email: user.email.to_owned(),
        password: user.password.to_owned()
    };
    let user_detail = db.get_user_by_email(&*user.email).unwrap();
    let pass_valid = verify(login_data.password, &*user_detail.password);
    if login_data.email == user_detail.email && pass_valid {
        let token = create_jwt(user_detail.id);
        return Ok(Json::from(CustomStatus {
            code: Status::Ok,
            message: token.unwrap(),
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

#[get("/auth")]
pub fn auth(key: Result<JWT, NetworkResponse>) -> Result<JWT, NetworkResponse> {
    let key = key;
    match key {
        Ok(key) => Ok(key),
        Err(_) => Err(NetworkResponse::BadRequest("e".to_string()))
    }
}