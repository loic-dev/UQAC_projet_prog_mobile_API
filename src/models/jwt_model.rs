use rocket::serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub subject_id: i32,
    pub(crate) exp: usize,
}

#[derive(Debug, Serialize)]
pub struct JWT {
    pub claims: Claims,
}