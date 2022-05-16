use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    HttpResponse, ResponseError,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use super::model::{Credentials, NewUser};

#[derive(Deserialize)]
pub struct SignUpForm {
    pub username: String,
    pub about: Option<String>,
    pub email: String,
    pub password: String,
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

impl TryFrom<SignUpForm> for NewUser {
    type Error = SignUpError;

    fn try_from(form: SignUpForm) -> Result<Self, Self::Error> {
        let salt = SaltString::generate(&mut OsRng).to_string();
        let pwd_hash = Secret::new(
            Argon2::default()
                .hash_password(form.password.as_bytes(), &salt)?
                .to_string(),
        );
        let credentials = Credentials::new(form.email, pwd_hash, salt)?;
        credentials.validate()?;
        let new_user = Self::new(form.username, form.about, credentials);
        new_user.validate()?;
        Ok(new_user)
    }
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
    fn status_code(&self) -> StatusCode {
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
            Self::AlreadyExist(v) => write!(f, "This {} is already taken.", v),
            Self::Unexpected(_) => write!(f, "Unexpected error happened."),
        }
    }
}

impl std::fmt::Debug for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "{}", e),
            Self::AlreadyExist(v) => write!(f, "This {} is already taken.", v),
            Self::Unexpected(e) => write!(f, "{}", e),
        }
    }
}

impl From<argon2::password_hash::Error> for SignUpError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => Self::Unexpected(Box::new(e)),
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
    pool: Data<PgPool>,
    form: Json<SignUpForm>,
) -> Result<HttpResponse, SignUpError> {
    let new_user = form.into_inner().try_into()?;
    let mut transaction = pool.begin().await?;
    let user_uuid = insert_user(&mut transaction, &new_user).await?;
    save_credentials(&mut transaction, &new_user, user_uuid).await?;
    transaction.commit().await?;
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
    sqlx::query!(
        r#"
            INSERT INTO credentials (owner_uuid, email, pwd_hash, salt)
                VALUES ($1, $2, $3, $4)
        "#,
        user_uuid,
        user.credentials.email,
        user.credentials.pwd_hash.expose_secret().to_string(),
        user.credentials.salt,
    )
    .execute(transaction)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::StatusCode,
        web::{Data, Json},
        ResponseError,
    };

    use super::{signup, SignUpForm};
    use crate::{
        config::test_config::configure_db,
        utils::test_utils::{
            lazy_init_subscriber, random_ascii_string, random_valid_email, random_valid_username,
        },
    };

    fn random_valid_signup_form() -> SignUpForm {
        SignUpForm {
            username: random_valid_username(),
            about: None,
            email: random_valid_email(),
            password: random_ascii_string(6..32),
        }
    }

    fn random_invalid_signup_form() -> SignUpForm {
        SignUpForm {
            username: random_ascii_string(1..64),
            about: None,
            email: random_ascii_string(1..64),
            password: random_ascii_string(1..64),
        }
    }

    #[actix_web::test]
    async fn signup_works() {
        lazy_init_subscriber();

        let pool = Data::new(
            configure_db()
                .await
                .expect("Failed to configure test database"),
        );
        let form = Json(random_valid_signup_form());
        let resp = signup(pool, form).await;
        assert_eq!(resp.unwrap().status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn signup_fails_on_username_validation_error() {
        lazy_init_subscriber();

        let pool = Data::new(
            configure_db()
                .await
                .expect("Failed to configure test database"),
        );
        let form = Json(random_invalid_signup_form());
        let resp = signup(pool, form).await;
        assert_eq!(resp.err().unwrap().status_code(), StatusCode::BAD_REQUEST);
    }
}
