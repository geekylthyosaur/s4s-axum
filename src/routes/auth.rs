use axum::{extract::State, Json};
use tracing::instrument;

use crate::{
    auth::jwt,
    dto::{AuthBody, LoginForm, SignupForm},
    error::ApiResult,
    extractors::ValidatedJson,
    service::Auth,
    storage::DbPool,
};

#[instrument(skip(pool))]
pub async fn signup(
    State(pool): State<DbPool>,
    ValidatedJson(form): ValidatedJson<SignupForm>,
) -> ApiResult<Json<AuthBody>> {
    let id = Auth::signup(&pool, form).await?;

    let token = jwt::Claims::new(id).sign()?;

    Ok(Json(AuthBody::new(token)))
}

#[instrument(skip(pool))]
pub async fn login(
    State(pool): State<DbPool>,
    ValidatedJson(form): ValidatedJson<LoginForm>,
) -> ApiResult<Json<AuthBody>> {
    let id = Auth::login(&pool, form).await?;

    let token = jwt::Claims::new(id).sign()?;

    Ok(Json(AuthBody::new(token)))
}
