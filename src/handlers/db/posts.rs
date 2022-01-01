use sqlx::{SqlitePool};

use crate::models::post::Post;

pub async fn get_posts(conn: &SqlitePool) -> Result<Vec<Post>, ()> {
    let res = sqlx::query!("
            SELECT * FROM posts
    ")
    .fetch_all(conn)
    .await
    .unwrap();

    let mut result = vec![];
    for rec in res {
        result.push(Post { title: rec.title, content: rec.content });
    }

    Ok(result)
}

pub async fn get_post(conn: &SqlitePool, id: u32) -> Result<Post, ()> {
    let record = sqlx::query!("
            SELECT * FROM posts
            WHERE id = $1
        ", id)
    .fetch_optional(conn)
    .await
    .unwrap();

    match record {
        Some(r) => {
            Ok(Post {
                title: r.title,
                content: r.content,
            })  
        },
        None => Err(()),
    }
}

pub async fn create_post(conn: &SqlitePool, post: Post) -> Result<(), ()> {
    sqlx::query!("
            INSERT INTO posts 
            ( title, content ) 
            VALUES ( $1, $2 )
        ", post.title, post.content)
    .execute(conn)
    .await
    .unwrap();
    
    Ok(())
}

pub async fn edit_post(conn: &SqlitePool, id: u32, post: Post) -> Result<(), ()> {
    sqlx::query!("
            UPDATE posts 
            SET title = $1,
                content = $2
            WHERE id = $3
        ", post.title, post.content, id)
    .execute(conn)
    .await
    .unwrap();
    Ok(())
}

pub async fn delete_post(conn: &SqlitePool, id: u32) -> Result<(), ()> {
    sqlx::query!("
            DELETE FROM posts WHERE id = $1
        ", id)
    .execute(conn)
    .await
    .unwrap();

    Ok(())
}
