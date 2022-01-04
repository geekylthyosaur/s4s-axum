use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::score::{Score, ScoreToCreate},
    error::Error,
};

pub async fn get_score(pool: &PgPool, uuid: Uuid) -> Result<Score, Error> {
    Ok(
        sqlx::query_as!(Score, r#"
                SELECT *
                FROM scores
                WHERE uuid = $1
            "#, uuid)
        .fetch_one(pool)
        .await?
    )
}

pub async fn create_score(pool: &PgPool, score: ScoreToCreate) -> Result<(), Error> {
    sqlx::query!(r#"
            INSERT INTO scores (uuid) 
            VALUES ($1)
        "#, score.uuid)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn increase_score(pool: &PgPool, uuid: Uuid) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE scores 
            SET score = score + 1
            WHERE uuid = $1
        "#, uuid)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn decrease_score(pool: &PgPool, uuid: Uuid) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE scores 
            SET score = score -1
            WHERE uuid = $1
        "#, uuid)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_score(pool: &PgPool, uuid: Uuid) -> Result<(), Error> {
    sqlx::query!(r#"
            DELETE FROM scores 
            WHERE uuid = $1
        "#, uuid)
    .execute(pool)
    .await?;

    Ok(())
}
