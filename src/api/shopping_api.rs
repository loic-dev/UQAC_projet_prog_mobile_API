use chrono::Utc;
use chrono_tz::America::Montreal;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::models::jwt_model::JWT;
use crate::models::shopping_model::ShoppingList;
use crate::models::status_model::{FailureResponse, SuccessResponse};
use crate::repository::mongodb_repo::MongoRepo;

#[post("/addlist", data="<new_list>")]
pub fn create_list(db: &State<MongoRepo>,
                   new_list: Json<ShoppingList>,
                   key: JWT,
) -> Result<Json<SuccessResponse<String>>, Json<FailureResponse>> {
    let current_time = Utc::now();
    let qc_time = current_time.with_timezone(&Montreal);
    let user_id = key.claims.subject_id;

    let data = ShoppingList {
        id: None,
        title: new_list.title.to_owned(),
        user: Some(user_id.to_owned()),
        created: Some(qc_time.to_rfc3339()),
        modified: Some(qc_time.to_rfc3339()),
        list: new_list.list.to_vec(),
    };
    let list_detail = db.create_list(data);
    match list_detail {
        Ok(_) => Ok(Json::from(SuccessResponse {
            code: Status::Ok,
            message: ("Shopping list has been created successfully").to_string(),
        })),
        Err(_) => Err(Json::from(FailureResponse {
            code: Status::InternalServerError,
            error: ("Internal Server Error").to_string(),
        }))
    }
}

#[get("/getlist/<path>")]
pub fn get_list(db: &State<MongoRepo>,
                path: String,
                _key: JWT) -> Result<Json<SuccessResponse<ShoppingList>>, Json<FailureResponse>> {
    let id = path;
    if id.is_empty() {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: "Id is null".to_string()
        }))
    };
    let list_detail = db.get_list(&id);
    return match list_detail {
        Ok(list) =>
            Ok(Json::from(SuccessResponse {
                code: Status::Accepted,
                message: list
            })),
        Err(_) =>
            Err(Json::from(FailureResponse {
                code: Status::InternalServerError,
                error: "Internal Server Error".to_string()
            })),
    }
}

#[put("/list/<path>", data = "<new_list>")]
pub fn update_list(
    db: &State<MongoRepo>,
    path: String,
    new_list: Json<ShoppingList>,
    key: JWT,
) -> Result<Json<SuccessResponse<ShoppingList>>, Json<FailureResponse>> {
    let id = path;
    if id.is_empty() {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: "Id is null".to_string()
        }))
    };
    let current_time = Utc::now();
    let qc_time = current_time.with_timezone(&Montreal);
    let user_id = key.claims.subject_id;
    let existing_list_result = db.get_list(&id);
    let existing_list = match existing_list_result {
        Ok(existing_list) => existing_list,
        Err(_) => {
            return Err(Json::from(FailureResponse {
                code: Status::NotFound,
                error: "List not found".to_string(),
            }));
        }
    };

    let data = ShoppingList {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_list.title.to_owned(),
        user: Some(user_id.to_owned()),
        created: existing_list.created,
        modified: Some(qc_time.to_rfc3339()),
        list: new_list.list.to_vec(),
    };
    let update_result = db.update_list(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_list_info = db.get_list(&id);
                return match updated_list_info {
                    Ok(list) =>
                        Ok(Json::from(SuccessResponse {
                            code: Status::Accepted,
                            message: list
                        })),
                    Err(_) =>
                        Err(Json::from(FailureResponse {
                            code: Status::InternalServerError,
                            error: "Internal Server Error".to_string()
                        })),
                };
            } else {
                return Err(Json::from(FailureResponse {
                    code: Status::NotFound,
                    error: "List Not Found".to_string()
                }))
            }
        }
        Err(_) =>
            Err(Json::from(FailureResponse {
                code: Status::InternalServerError,
                error: "Internal Server Error".to_string()
            }))
    }
}

#[get("/lists")]
pub fn get_lists(db: &State<MongoRepo>,
                 key: JWT
) -> Result<Json<SuccessResponse<Vec<ShoppingList>>>, Json<FailureResponse>> {
    let user_id = key.claims.subject_id;
    let lists = db.get_lists(&user_id);
    match lists {
        Ok(lists) => Ok(Json::from(SuccessResponse {
            code: Status::Accepted,
            message: lists
        })),
        Err(_) => Err(Json::from(FailureResponse {
            code: Status::InternalServerError,
            error: "Internal Server Error".to_string()
        }))

    }
}

#[delete("/list/<path>")]
pub fn delete_list(db: &State<MongoRepo>,
                   path: String,
                   key: JWT) -> Result<Json<SuccessResponse<String>>, Json<FailureResponse>> {
    let id = path;
    if id.is_empty() {
        return Err(Json::from(FailureResponse {
            code: Status::BadRequest,
            error: "Id is null".to_string()
        }))
    };
    if db.get_list_for_user(&id, &key.claims.subject_id).is_none() {
        return Err(Json::from(FailureResponse {
            code: Status::NotFound,
            error: "Shopping list not found".to_string()
        }))
    }
    let result = db.delete_list(&id);
    return match result {
        Ok(_) =>
            Ok(Json::from(SuccessResponse {
                code: Status::Accepted,
                message: "List successfully deleted".to_string()
            })),
        Err(_) =>
            Err(Json::from(FailureResponse {
                code: Status::InternalServerError,
                error: "Internal Server Error while deleting list".to_string()
            })),
    }
}