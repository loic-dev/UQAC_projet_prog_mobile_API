use std::env;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, DeleteResult},
    sync::{Client, Collection},
};
use mongodb::results::UpdateResult;
use crate::models::shopping_model::ShoppingList;
use crate::models::user_model::User;

pub struct MongoRepo {
    col_user: Collection<User>,
    col_list: Collection<ShoppingList>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        println!("{:?}", uri);
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col_user: Collection<User> = db.collection("User");
        let col_list: Collection<ShoppingList> = db.collection("Shopping List");
        MongoRepo { col_user, col_list }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            email: new_user.email,
            password: new_user.password,
        };
        println!("{:?}", new_doc);
        let user = self
            .col_user
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col_user
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<User> {
        let filter = doc! {"email": email};
        let user_detail = self
            .col_user
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        return user_detail;
    }


    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col_user
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn create_list(&self, new_list: ShoppingList) -> Result<InsertOneResult, Error> {
        let new_doc = ShoppingList {
            id: None,
            title: new_list.title,
            user: new_list.user,
            created: new_list.created,
            modified: new_list.modified,
            list: new_list.list,
        };
        println!("{:?}", new_doc);
        let shopping_list = self
            .col_list
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating shopping list");
        Ok(shopping_list)
    }

    pub fn get_list(&self, id: &String) -> Result<ShoppingList, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let list_detail = self
            .col_list
            .find_one(filter, None)
            .ok()
            .expect("Error getting list's detail");
        Ok(list_detail.unwrap())
    }

    pub fn update_list(&self, id: &String, new_list: ShoppingList) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_list.id,
                    "title": new_list.title,
                    "user": new_list.user,
                    "created": new_list.created,
                    "modified": new_list.modified,
                    "list": new_list.list,
                },
        };
        let updated_doc = self
            .col_list
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating list");
        Ok(updated_doc)
    }
}