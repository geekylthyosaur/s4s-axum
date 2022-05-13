use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use validator::{Validate, ValidationErrors};
use sqlx::{PgPool, Postgres, Transaction};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::user::model::NewUser;

#[derive(Serialize, Deserialize)]
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
    type Error = ValidationErrors;

    fn try_from(form: SignUpForm) -> Result<Self, Self::Error> {
        let new_user = Self::new(form.username, form.about, form.email, form.password);
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

#[cfg(test)]
mod tests {
    use actix_web::{
        ResponseError,
        web::{Data, Json},
    };

    use crate::{
        utils::test_utils::lazy_init_subscriber,
        config::test_config::configure_db,
    };
    use super::{SignUpForm, signup};

    #[actix_web::test]
    async fn signup_works() {
        lazy_init_subscriber();

        let pool = Data::new(
            configure_db()
            .await
            .expect("Failed to configure test database")
        );
        let form = Json(SignUpForm {
            username: "username".to_string(),
            about: None,
            email: "example@email.com".to_string(),
            password: "password".to_string(),
        });
        let resp = signup(pool, form).await;
        assert_eq!(resp.unwrap().status(), actix_web::http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn signup_fails_on_username_validation_error() {
        lazy_init_subscriber();

        let pool = Data::new(
            configure_db()
            .await
            .expect("Failed to configure test database")
        );
        let form = Json(SignUpForm {
            username: "USERNAME".to_string(),
            about: None,
            email: "example@email.com".to_string(),
            password: "password".to_string(),
        });
        let resp = signup(pool, form).await;
        assert_eq!(resp.err().unwrap().status_code(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}
