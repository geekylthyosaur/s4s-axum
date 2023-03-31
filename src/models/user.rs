use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    #[serde(skip_serializing)]
    id: Uuid,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    age: u8,
    about: Option<String>,
    verified: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
