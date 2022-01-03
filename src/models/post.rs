use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
