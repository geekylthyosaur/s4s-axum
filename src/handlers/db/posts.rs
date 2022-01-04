use sqlx::PgPool;

use crate::{
    models::post::{Post, PostToCreate, PostToUpdate},
    error::Error,
};

pub async fn get_posts(pool: &PgPool) -> Result<Vec<Post>, Error> {
    Ok(
        sqlx::query_as!(Post, "
                SELECT *
                FROM posts
            ")
        .fetch_all(pool)
        .await?
    )
}

pub async fn get_post(pool: &PgPool, id: i32) -> Result<Post, Error> {
    Ok(
        sqlx::query_as!(Post, r#"
                SELECT *
                FROM posts
                WHERE id = $1
            "#, id)
        .fetch_one(pool)
        .await?
    )
}

pub async fn create_post(pool: &PgPool, post: PostToCreate) -> Result<(), Error> {
    let uuid = uuid::Uuid::new_v4();
    sqlx::query!(r#"
            INSERT INTO posts (uuid, owner_id, title, content) 
            VALUES ($1, $2, $3, $4)
        "#, uuid, post.owner_id, post.title, post.content)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn edit_post(pool: &PgPool, id: i32, post: PostToUpdate) -> Result<(), Error> {
    let time = chrono::Utc::now();
    sqlx::query!(r#"
            UPDATE posts 
            SET title = $1, content = $2, date_updated = $3
            WHERE id = $4
        "#, post.title, post.content, time, id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_post(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"
            DELETE FROM posts 
            WHERE id = $1
        "#, id)
    .execute(pool)
    .await?;

    Ok(())
}
