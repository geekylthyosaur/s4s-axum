use sqlx::SqlitePool;

use crate::{
    models::post::Post,
    error::Error,
};

pub async fn get_posts(pool: &SqlitePool) -> Result<Vec<Post>, Error> {
    Ok(
        sqlx::query_as!(Post, "
                SELECT title, content
                FROM posts
            ")
        .fetch_all(pool)
        .await?
    )
}

pub async fn get_post(pool: &SqlitePool, id: u32) -> Result<Post, Error> {
    Ok(
        sqlx::query_as!(Post, r#"
                SELECT title, content
                FROM posts
                WHERE id = $1
            "#, id)
        .fetch_one(pool)
        .await?
    )
}

pub async fn create_post(pool: &SqlitePool, post: Post) -> Result<(), Error> {
    sqlx::query!(r#"
            INSERT INTO posts (title, content) 
            VALUES ($1, $2)
        "#, post.title, post.content)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn edit_post(pool: &SqlitePool, id: u32, post: Post) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE posts 
            SET title = $1, content = $2
            WHERE id = $3
        "#, post.title, post.content, id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_post(pool: &SqlitePool, id: u32) -> Result<(), Error> {
    sqlx::query!(r#"
            DELETE FROM posts 
            WHERE id = $1
        "#, id)
    .execute(pool)
    .await?;

    Ok(())
}
