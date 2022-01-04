use sqlx::PgPool;

use crate::{
    models::user::{User, UserToCreate, UserToUpdate},
    error::Error,
};

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    Ok(
        sqlx::query_as!(User, "
                SELECT *
                FROM users
            ")
        .fetch_all(pool)
        .await?
    )
}

pub async fn get_user(pool: &PgPool, id: i32) -> Result<User, Error> {
    Ok(
        sqlx::query_as!(User, r#"
                SELECT *
                FROM users
                WHERE id = $1
            "#, id)
        .fetch_one(pool)
        .await?
    )
}

pub async fn create_user(pool: &PgPool, user: UserToCreate) -> Result<(), Error> {
    let time = chrono::Utc::now();
    let uuid = uuid::Uuid::new_v4();
    sqlx::query!(r#"
            INSERT INTO users (uuid, username, email, passwd_hash, about, join_date) 
            VALUES ($1, $2, $3, $4, $5, $6)
        "#, uuid, user.username, user.email, user.passwd_hash, user.about, time)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn edit_user(pool: &PgPool, id: i32, user: UserToUpdate) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE users 
            SET username = $2, email = $3, passwd_hash = $4, about = $5
            WHERE id = $1
        "#, id, user.username, user.email, user.passwd_hash, user.about)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"
            DELETE FROM users 
            WHERE id = $1
        "#, id)
    .execute(pool)
    .await?;

    Ok(())
}
