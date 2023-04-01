use axum::{extract::State, Json};

use crate::{
    auth::jwt,
    dto::{AuthBody, LoginForm, SignupForm},
    error::ApiResult,
    extractors::ValidatedJson,
    service::AuthService,
    storage::DbPool,
};

#[axum_macros::debug_handler]
pub async fn signup(
    State(pool): State<DbPool>,
    ValidatedJson(form): ValidatedJson<SignupForm>,
) -> ApiResult<Json<AuthBody>> {
    let id = AuthService::signup(&pool, form).await?;

    let token = jwt::Claims::new(id).sign()?;

    Ok(Json(AuthBody::new(token)))
}

#[axum_macros::debug_handler]
pub async fn login(
    State(pool): State<DbPool>,
    ValidatedJson(form): ValidatedJson<LoginForm>,
) -> ApiResult<Json<AuthBody>> {
    let id = AuthService::login(&pool, form).await?;

    let token = jwt::Claims::new(id).sign()?;

    Ok(Json(AuthBody::new(token)))
}
