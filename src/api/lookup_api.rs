use rocket::http::Status;
use rocket::serde::json::Json;
use crate::models::jwt_model::JWT;
use crate::models::status_model::ReqwestStatus;

#[get("/search/<upc>")]
pub async fn lookup(upc: &str, _key: JWT) -> Result<Json<ReqwestStatus>, String> {
    let url = format!("https://api.upcitemdb.com/prod/trial/lookup?upc={}", upc);
    let response = reqwest::get(&url).await
        .map_err(|err|format!("Error getting url content : {}", err))?
        .json::<serde_json::Value>()
        .await
        .map_err(|err|format!("Error getting url content : {}", err))?;
    Ok(Json::from(ReqwestStatus {
        code: Status::Ok,
        message: response
    }))
}