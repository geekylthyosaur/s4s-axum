use sqlx::PgPool;

use crate::{
    models::user::{User, UserToCreate, UserToUpdate},
    error::Error,
};

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, Error> {
    Ok(
        sqlx::query_as!(User, r#"
                SELECT *
                FROM users
            "#)
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
    let mut tr = pool.begin().await?;
    let id: i32 = sqlx::query!(r#"
            INSERT INTO users (username, about) 
                VALUES ($1, $2)
            RETURNING id
        "#, user.username, user.about)
    .fetch_one(&mut tr)
    .await?
    .id;

    sqlx::query!(r#"
            INSERT INTO credentials (owner_id, email, pwd_hash) 
                VALUES ($1, $2, $3)
        "#, id, user.email, user.pwd_hash)
    .execute(&mut tr)
    .await?;
    
    tr.commit().await?;

    Ok(())
}

pub async fn edit_user(pool: &PgPool, id: i32, user: UserToUpdate) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE users 
            SET username = $2, about = $3
            WHERE id = $1
        "#, id, user.username, user.about)
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
