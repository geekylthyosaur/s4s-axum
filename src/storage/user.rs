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

pub async fn create(pool: &DbPool, user: User) -> SqlxResult<()> {
    sqlx::query!(
        r#"
            INSERT INTO users (id, first_name, last_name, username, email, pwd_hash, age, about, verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        "#,
        user.id,
        user.first_name,
        user.last_name,
        user.username,
        user.email,
        user.pwd_hash,
        user.age,
        user.about,
        user.verified,
        user.created_at,
        user.updated_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}
