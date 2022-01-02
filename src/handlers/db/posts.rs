use sqlx::{SqlitePool};

use crate::models::post::Post;
use crate::error::Error;

pub async fn get_posts(conn: &SqlitePool) -> Result<Vec<Post>, Error> {
    let res = sqlx::query!("
            SELECT * FROM posts
        ")
    .fetch_all(conn)
    .await?;

    let mut result = vec![];
    for rec in res {
        result.push(Post { title: rec.title, content: rec.content });
    }

    Ok(result)
}

pub async fn get_post(conn: &SqlitePool, id: u32) -> Result<Post, Error> {
    let record = sqlx::query!(r#"
            SELECT * FROM posts
            WHERE id = $1
        "#, id)
    .fetch_one(conn)
    .await?;

    Ok(Post { title: record.title, content: record.content })
}

pub async fn create_post(conn: &SqlitePool, post: Post) -> Result<(), Error> {
    sqlx::query!(r#"
            INSERT INTO posts 
            ( title, content ) 
            VALUES ( $1, $2 )
        "#, post.title, post.content)
    .execute(conn)
    .await?;
    
    Ok(())
}

pub async fn edit_post(conn: &SqlitePool, id: u32, post: Post) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE posts 
            SET title = $1,
                content = $2
            WHERE id = $3
        "#, post.title, post.content, id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn delete_post(conn: &SqlitePool, id: u32) -> Result<(), Error> {
    sqlx::query!(r#"
            DELETE FROM posts WHERE id = $1
        "#, id)
    .execute(conn)
    .await?;

    Ok(())
}
