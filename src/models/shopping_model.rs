use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    pub list: Vec<ListItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListItem {
    pub name: String,
    pub checked: bool,
}

use bson::{Bson, doc};

impl From<ListItem> for Bson {
    fn from(item: ListItem) -> Self {
        let doc = doc! {
            "name": item.name,
            "checked": item.checked,
        };

        Bson::Document(doc)
    }
}
