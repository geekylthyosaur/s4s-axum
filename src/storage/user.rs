use sqlx::Result as SqlxResult;
use uuid::Uuid;

use crate::models::user::User;

use super::DbPool;

pub async fn get_by_id(pool: &DbPool, id: Uuid) -> SqlxResult<Option<User>> {
    todo!()
}

pub async fn get_by_email(pool: &DbPool, email: String) -> SqlxResult<Option<User>> {
    todo!()
}

pub async fn get_by_username(pool: &DbPool, username: String) -> SqlxResult<Option<User>> {
    todo!()
}
