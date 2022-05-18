use actix_web::{http::StatusCode, ResponseError};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use super::model::Credentials;

pub enum AuthError {
    InvalidCredentials,
    Unexpected(Box<dyn std::error::Error>),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "Invalid email or password."),
            Self::Unexpected(_) => write!(f, "Unexpected error happened"),
        }
    }
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "Invalid email or password."),
            Self::Unexpected(e) => write!(f, "{}", e),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<argon2::Error> for AuthError {
    fn from(e: argon2::Error) -> Self {
        match e {
            _ => AuthError::Unexpected(Box::new(e)),
        }
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            argon2::password_hash::Error::Password => Self::InvalidCredentials,
            _ => AuthError::Unexpected(Box::new(e)),
        }
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e)
                if e.downcast_ref::<sqlx::postgres::PgDatabaseError>().code() == "23505" =>
            {
                Self::InvalidCredentials
            }
            _ => Self::Unexpected(Box::new(e)),
        }
    }
}

#[tracing::instrument(name = "Validation of user credentials", skip(pool, credentials))]
pub async fn validate_credentials(
    pool: &PgPool,
    credentials: Credentials,
) -> Result<Uuid, AuthError> {
    let row = sqlx::query!(
        r#"
        SELECT owner_uuid, pwd_hash, salt
        FROM credentials
        WHERE email=$1
    "#,
        credentials.email
    )
    .fetch_one(pool)
    .await?;
    let candidate = credentials.calc_pwd_hash(&row.salt)?;
    let expected = Secret::new(row.pwd_hash);

    verify_password_hash(expected, candidate)?;

    Ok(row.owner_uuid)
}

#[tracing::instrument(name = "Password hash verification", skip(expected, candidate))]
fn verify_password_hash(
    expected: Secret<String>,
    candidate: Secret<String>,
) -> Result<(), AuthError> {
    if expected.expose_secret() == candidate.expose_secret() {
        return Ok(());
    }
    Err(AuthError::InvalidCredentials)
}
