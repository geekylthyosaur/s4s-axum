use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub uuid: Uuid,
    pub owner_id: i32,
    pub title: String,
    pub content: String,
    pub date_created: DateTime::<Utc>,
    pub date_updated: Option<DateTime::<Utc>>,
}

#[derive(Deserialize)]
pub struct PostToCreate {
    pub owner_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostToUpdate {
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
