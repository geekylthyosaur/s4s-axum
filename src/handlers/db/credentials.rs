use sqlx::PgPool;

use crate::{
    models::credentials::{Credentials, CredentialsToUpdate},
    error::Error,
};

pub async fn get_credentials(pool: &PgPool, owner_id: i32) -> Result<Credentials, Error> {
    Ok(
        sqlx::query_as!(Credentials, r#"
                SELECT *
                FROM credentials
                WHERE owner_id = $1
            "#, owner_id)
        .fetch_one(pool)
        .await?
    )
}

pub async fn edit_credentials(pool: &PgPool, owner_id: i32, credentials: CredentialsToUpdate) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE credentials
            SET email = $1, pwd_hash = $2
            WHERE owner_id = $3
        "#, credentials.email, credentials.pwd_hash, owner_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn toggle_email_confirm(pool: &PgPool, owner_id: i32) -> Result<(), Error> {
    sqlx::query!(r#"
            UPDATE credentials
            SET is_email_confirmed = NOT is_email_confirmed
            WHERE owner_id = $1
        "#, owner_id)
    .execute(pool)
    .await?;

    Ok(())
}
