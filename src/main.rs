mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use repository::mongodb_repo::MongoRepo;
use crate::api::user_api::{create_user, get_user, update_user, delete_user, get_all_users, login_user, auth};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    dotenv().ok();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/" ,routes![login_user])
        .mount("/", routes![auth])
}