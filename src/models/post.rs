use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
}
