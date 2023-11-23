use rocket::State;
use crate::MongoRepo;

pub fn is_valid_password(password: &str) -> bool {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special_char = password.chars().any(|c| "!@#$&*".contains(c));
    let has_min_length = password.len() >= 8;

    has_uppercase && has_lowercase && has_digit && has_special_char && has_min_length
}

pub fn email_exist(db: &State<MongoRepo>, email: &str) -> bool {
    let mut result = false;
    let user = db.get_user_by_email(email);
    if user.is_some() {result = true}
    result
}