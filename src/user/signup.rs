use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::user::model::NewUser;

#[derive(Deserialize)]
pub struct SignUpForm {
    pub username: String,
    pub about: Option<String>,
    pub email: String,
    pub password: String,
}

impl TryFrom<SignUpForm> for NewUser {
    type Error = ValidationErrors;

    fn try_from(form: SignUpForm) -> Result<Self, Self::Error> {
        let new_user = Self::new(form.username, form.about, form.email, form.password);
        new_user.validate()?;
        Ok(new_user)
    }
}

pub enum SignUpError {
    Validation(ValidationErrors),
    AlreadyExist(UniqueField),
    Unexpected(Box<dyn std::error::Error>),
}

pub enum UniqueField {
    Email,
    Username,
}

impl std::fmt::Display for UniqueField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Username => write!(f, "username"),
        }
    }
}

impl ResponseError for SignUpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SignUpError::Validation(_) => StatusCode::BAD_REQUEST,
            SignUpError::AlreadyExist(_) => StatusCode::CONFLICT,
            SignUpError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ValidationErrors> for SignUpError {
    fn from(e: ValidationErrors) -> Self {
        Self::Validation(e)
    }
}

impl From<sqlx::Error> for SignUpError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e)
                if e.downcast_ref::<sqlx::postgres::PgDatabaseError>().code() == "23505" =>
            {
                match e.constraint() {
                    Some("credentials_email_key") => SignUpError::AlreadyExist(UniqueField::Email),
                    Some("users_username_key") => SignUpError::AlreadyExist(UniqueField::Username),
                    _ => unreachable!(),
                }
            }
            _ => SignUpError::Unexpected(Box::new(e)),
        }
    }
}

impl std::fmt::Display for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "{}", e),
            Self::AlreadyExist(v) => write!(f, "This {} already taken.", v),
            Self::Unexpected(_) => write!(f, "Unexpected error happened."),
        }
    }
}

impl std::fmt::Debug for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "{}", e),
            Self::AlreadyExist(v) => write!(f, "This {} already taken.", v),
            Self::Unexpected(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for SignUpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unexpected(e) => Some(&**e),
            _ => None,
        }
    }
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(form, pool),
    fields(
        email = %form.email,
        username = %form.username
    )
)]
pub async fn signup(
    pool: web::Data<PgPool>,
    form: web::Json<SignUpForm>,
) -> Result<HttpResponse, SignUpError> {
    let user = form.into_inner().try_into().map_err(|e| {
        tracing::error!("Failed to validate form data: {}", e);
        e
    })?;
    let mut transaction = pool.begin().await.map_err(|e| {
        tracing::error!(
            "Failed to acquire a Postgres connection from the pool: {}",
            e
        );
        e
    })?;
    let user_id = insert_user(&mut transaction, &user).await.map_err(|e| {
        tracing::error!("Failed to insert new user in the database: {}", e);
        e
    })?;
    save_credentials(&mut transaction, &user, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save the credentials for a new user: {}", e);
            e
        })?;
    transaction.commit().await.map_err(|e| {
        tracing::error!(
            "Failed to commit SQL transaction to store a new user: {}",
            e
        );
        e
    })?;
    Ok(HttpResponse::Created().finish())
}

#[tracing::instrument(
    name = "Saving new user details in the database",
    skip(transaction, user)
)]
async fn insert_user(
    transaction: &mut Transaction<'_, Postgres>,
    user: &NewUser,
) -> Result<Uuid, sqlx::Error> {
    let user_uuid = sqlx::query!(
        r#"
            INSERT INTO users (uuid, username, about)
                VALUES ($1, $2, $3)
            RETURNING uuid
        "#,
        Uuid::new_v4(),
        user.username,
        user.about
    )
    .fetch_one(transaction)
    .await?
    .uuid;

    Ok(user_uuid)
}

#[tracing::instrument(
    name = "Saving new user credentials in the database",
    skip(transaction, user, user_uuid)
)]
async fn save_credentials(
    transaction: &mut Transaction<'_, Postgres>,
    user: &NewUser,
    user_uuid: Uuid,
) -> Result<(), sqlx::Error> {
    // TODO: password auth https://www.lpalmieri.com/posts/password-authentication-in-rust/
    sqlx::query!(
        r#"
            INSERT INTO credentials (owner_uuid, email, pwd_hash)
                VALUES ($1, $2, $3)
        "#,
        user_uuid,
        user.email,
        user.password
    )
    .execute(transaction)
    .await?;

    Ok(())
}
