mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use repository::mongodb_repo::MongoRepo;
use crate::api::user_api::{get_user, update_user, delete_user, auth, jwt_unauthorized};
use crate::api::login_api::login_user;
use crate::api::lookup_api::lookup;
use crate::api::register_api::create_user;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    dotenv().ok();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/api", routes![get_user])
        .mount("/api", routes![update_user])
        .mount("/api", routes![delete_user])
        .mount("/" ,routes![login_user])
        .mount("/", routes![auth])
        .mount("/api", routes![lookup])
        .register("/api", catchers![jwt_unauthorized])
}