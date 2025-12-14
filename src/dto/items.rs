use serde::{Deserialize, Serialize};
use crate::models::items::Model;

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: String
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl From<Model> for ItemResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
        }
    }
}