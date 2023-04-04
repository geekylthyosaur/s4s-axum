use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(sqlx::Error),
    #[error("The requested resource was not found.")]
    NotFound(sqlx::Error),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),
    #[error(transparent)]
    AxumJson(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AxumTypedHeader(#[from] axum::extract::rejection::TypedHeaderRejection),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error("Wrong credentials.")]
    WrongCredentials,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::Validation(_) | Error::AxumJson(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"error": {"message": err.to_string()}});
        (status, Json(payload))
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound(err),
            _ => Self::Sqlx(err),
        }
    }
}
