use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub about: Option<String>,
    pub join_date: DateTime::<Utc>,
}

#[derive(Deserialize)]
pub struct UserToCreate {
    pub username: String,
    pub about: Option<String>,
    pub email: String,
    pub pwd_hash: String,
}

#[derive(Deserialize)]
pub struct UserToUpdate {
    pub username: String,
    pub about: Option<String>,
}

impl User {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
