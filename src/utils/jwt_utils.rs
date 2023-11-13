use std::env;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use crate::models::jwt_model::Claims;
use crate::models::status_model::CustomStatus;
use jsonwebtoken::errors::ErrorKind;

pub fn create_jwt(id: Option<ObjectId>) -> Result<String, Json<CustomStatus>> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let bytes = id.unwrap().bytes();
    let id_int = i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let expiration= Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id_int,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    let jwt = encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()));
    let str_jwt = match jwt {
        Ok(token) => token,
        Err(err) => {
            format!("Error: {:?}", err)
        }
    };
    Ok(str_jwt)
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
