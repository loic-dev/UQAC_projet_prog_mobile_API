use std::fs::{File, remove_file};
use std::io::{Read, Write};
use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::Value;
use crate::models::jwt_model::JWT;
use crate::models::status_model::ReqwestStatus;

#[post("/shop/upload", data = "<invoice>")]
pub async fn upload_list(
    invoice: Json<Value>,
    key: JWT) -> Result<Json<ReqwestStatus>, String> {
    let storage_url = "https://firebasestorage.googleapis.com/v0/b/uqacprogmobilefirebase.appspot.com/o";


    let json_bytes = invoice.into_inner().to_string().into_bytes();

    let user_id = key.claims.subject_id;
    let file_id = Utc::now().timestamp();
    let file_path = format!("ProductList-{}.txt", file_id);
    let mut file = File::create(&file_path)
        .map_err(|err| format!("Error creating file: {}", err))?;

    file.write_all(&json_bytes)
        .map_err(|err| format!("Error writing to file: {}", err))?;

    let mut file = File::open(&file_path)
        .map_err(|err|format!("Error getting url content : {}", err))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|err|format!("Error getting url content : {}", err))?;

    let storage_path = format!("{}%2FProductList-{}.txt", user_id, file_id);
    let upload_url = format!(
        "{}/{}?alt=media",
        storage_url,
        storage_path
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&upload_url)
        .body(buffer)
        .send().await
        .map_err(|err|format!("Error getting url content : {}", err))?
        .json::<Value>().await
        .map_err(|err|format!("Error getting url content : {}", err))?;

    remove_file(file_path).expect("Error removing file");

    Ok(Json::from(ReqwestStatus {
        code: Status::Ok,
        message: response
    }))
}

#[get("/shop/list")]
pub async fn get_shop_list_user(key: JWT) -> Result<Json<ReqwestStatus>, String> {
    let user_id = key.claims.subject_id;
    let storage_url = "https://firebasestorage.googleapis.com/v0/b/uqacprogmobilefirebase.appspot.com/o";

    let storage_path = format!("{}/?prefix={}/", storage_url, user_id);
    println!("{:?}", storage_path);

    let response = reqwest::get(&storage_path).await
        .map_err(|err|format!("Error getting url content : {}", err))?
        .json::<Value>()
        .await
        .map_err(|err|format!("Error getting url content : {}", err))?;

    Ok(Json::from(ReqwestStatus {
        code: Status::Ok,
        message: response
    }))
}