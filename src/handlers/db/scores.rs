use sqlx::PgPool;

use crate::{
    models::score::Score,
    error::Error,
};

pub async fn get_score(pool: &PgPool, id: i32) -> Result<Score, Error> {
    Ok(
        sqlx::query_as!(Score, r#"
                SELECT *
                FROM scores
                WHERE id = $1
            "#, id)
        .fetch_one(pool)
        .await?
    )
}

pub async fn increase_score(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE scores 
            SET score = score + 1
            WHERE id = $1
        "#, id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn decrease_score(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE scores 
            SET score = score -1
            WHERE id = $1
        "#, id)
    .execute(pool)
    .await?;

    Ok(())
}
