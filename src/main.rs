mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use repository::mongodb_repo::MongoRepo;
use crate::api::firebase_api::{get_shop_list_user, upload_list};
use crate::api::user_api::{get_user, delete_user, auth, jwt_unauthorized};
use crate::api::login_api::login_user;
use crate::api::lookup_api::{lookup, search};
use crate::api::register_api::create_user;
use crate::api::shopping_api::{create_list, get_list, get_lists, update_list, delete_list};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    dotenv().ok();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/" ,routes![login_user])
        .mount("/", routes![auth])
        .mount("/api", routes![get_user])
        .mount("/api", routes![delete_user])
        .mount("/api", routes![create_list])
        .mount("/api", routes![get_list])
        .mount("/api", routes![update_list])
        .mount("/api", routes![get_lists])
        .mount("/api", routes![delete_list])
        .mount("/api", routes![lookup])
        .mount("/api", routes![search])
        .mount("/api", routes![upload_list])
        .mount("/api", routes![get_shop_list_user])
        .register("/api", catchers![jwt_unauthorized])
}