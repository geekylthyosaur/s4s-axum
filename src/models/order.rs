use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Validate)]
pub struct Order {
    pub id: Uuid,
    pub student_id: Uuid,
    pub mentor_id: Uuid,
    #[validate(range(min = 0))]
    pub price: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
