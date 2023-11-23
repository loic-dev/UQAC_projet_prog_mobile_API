use std::env;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use crate::models::jwt_model::Claims;
use jsonwebtoken::errors::ErrorKind;

pub fn create_jwt(id: Option<ObjectId>) -> Result<String, String> {
    if id.is_none() {
        return Err("No id".to_string())
    }
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let bytes = id.unwrap().bytes();
    let id_int = i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let expiration= Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id_int,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    return match encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes())) {
        Ok(token) => Ok(token),
        Err(err) => {
            println!("Error: {:?}", err);
            Err("Internal Server Error while encoding token".to_string())
        }
    }


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