use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

use crate::validators::is_lowercase_alphabetic;

#[derive(Debug, Serialize, Validate)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Uuid,
    #[validate(length(min = 4, max = 16), custom = "is_lowercase_alphabetic")]
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    pub pwd_hash: String,
    #[validate(range(min = 0, max = 128))]
    pub age: Option<i32>,
    #[validate(length(max = 512))]
    pub about: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
