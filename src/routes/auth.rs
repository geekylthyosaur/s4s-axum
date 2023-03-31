use axum::{extract::State, http::StatusCode, Json};

use crate::{
    dto::{LoginForm, SignupForm},
    error::ApiResult,
    extractors::Validated,
    storage::DbPool, service::AuthService,
};

pub async fn signup(
    State(pool): State<DbPool>,
    Json(Validated(form)): Json<Validated<SignupForm>>,
) -> ApiResult<StatusCode> {
    AuthService::signup(&pool, form).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(Validated(form)): Json<Validated<LoginForm>>,
) -> ApiResult<StatusCode> {
    AuthService::login(&pool, form).await?;
    Ok(StatusCode::NO_CONTENT)
}
