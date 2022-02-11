use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};

use crate::user::model::NewUser;

#[derive(Deserialize)]
pub struct SignUpForm {
    username: String,
    about: Option<String>,
    email: String,
    password: String,
}

impl TryFrom<SignUpForm> for NewUser {
    type Error = SignUpError;

    fn try_from(form: SignUpForm) -> Result<Self, Self::Error> {
        Ok(NewUser {
            username: form.username,
            about: form.about,
            email: form.email,
            password: form.password,
        })
    }
}

pub enum SignUpError {
    Validation(String), // TODO: email validation
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
            Self::Validation(s) => write!(f, "Validation error: {}.", s),
            Self::AlreadyExist(v) => write!(f, "This {} already taken.", v),
            Self::Unexpected(_) => write!(f, "Unexpected error happened."),
        }
    }
}

impl std::fmt::Debug for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(s) => write!(f, "Validation error: {}.", s),
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
    let mut transaction = pool.begin().await.map_err(|e| {
        tracing::error!(
            "Failed to acquire a Postgres connection from the pool: {}",
            e
        );
        e
    })?;
    let user = form.into_inner().try_into()?;
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
    skip(transaction, user),
)]
async fn insert_user(
    transaction: &mut Transaction<'_, Postgres>,
    user: &NewUser,
) -> Result<i32, sqlx::Error> {
    // TODO: replace id with uuid
    let user_id = sqlx::query!(
        r#"
            INSERT INTO users (username, about) 
                VALUES ($1, $2)
            RETURNING id
        "#,
        user.username,
        user.about
    )
    .fetch_one(transaction)
    .await?
    .id;

    Ok(user_id)
}

#[tracing::instrument(
    name = "Saving new user credentials in the database",
    skip(transaction, user, user_id),
)]
async fn save_credentials(
    transaction: &mut Transaction<'_, Postgres>,
    user: &NewUser,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    // TODO: password auth https://www.lpalmieri.com/posts/password-authentication-in-rust/
    sqlx::query!(
        r#"
            INSERT INTO credentials (owner_id, email, pwd_hash) 
                VALUES ($1, $2, $3)
        "#,
        user_id,
        user.email,
        user.password
    )
    .execute(transaction)
    .await?;

    Ok(())
}
