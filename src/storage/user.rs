use sqlx::Result as SqlxResult;
use tracing::instrument;
use uuid::Uuid;

use crate::models::user::User;

use super::DbPool;

#[instrument(skip(pool))]
pub async fn get_all(pool: &DbPool) -> SqlxResult<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
            SELECT *
            FROM users;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

#[instrument(skip(pool))]
pub async fn get_by_id(pool: &DbPool, id: Uuid) -> SqlxResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT *
            FROM users
            WHERE users.id = $1;
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[instrument(skip(pool))]
pub async fn get_by_email(pool: &DbPool, email: String) -> SqlxResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT *
            FROM users
            WHERE users.email = $1;
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[instrument(skip(pool))]
pub async fn get_by_username(pool: &DbPool, username: String) -> SqlxResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT *
            FROM users
            WHERE users.username = $1;
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

#[instrument(skip(pool))]
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

#[instrument(skip(pool))]
pub async fn edit(pool: &DbPool, user: User) -> SqlxResult<()> {
    sqlx::query!(
        r#"
            UPDATE users 
            SET (first_name, last_name, username, age, about) = ($2, $3, $4, $5, $6)
            WHERE users.id = $1;
        "#,
        user.id,
        user.first_name,
        user.last_name,
        user.username,
        user.age,
        user.about,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn edit_email(pool: &DbPool, user: User) -> SqlxResult<()> {
    sqlx::query!(
        r#"
            UPDATE users 
            SET email = $2
            WHERE users.id = $1;
        "#,
        user.id,
        user.email,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn edit_password(pool: &DbPool, user: User) -> SqlxResult<()> {
    sqlx::query!(
        r#"
            UPDATE users 
            SET pwd_hash = $2
            WHERE users.id = $1;
        "#,
        user.id,
        user.pwd_hash,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn delete(pool: &DbPool, id: Uuid) -> SqlxResult<()> {
    sqlx::query!(
        r#"
            DELETE FROM users
            WHERE users.id = $1;
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
