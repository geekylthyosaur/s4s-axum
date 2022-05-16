use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;

use super::auth::{validate_credentials, AuthError};

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    email: String,
    password: Secret<String>,
}

#[tracing::instrument(
    name = "Logging in",
    skip(form, pool),
    fields(
        email = %form.email,
    )
)]
pub async fn login(pool: Data<PgPool>, form: Json<LoginForm>) -> Result<HttpResponse, AuthError> {
    let _uuid = validate_credentials(&pool, form.email.clone(), form.password.clone()).await?;
    Ok(HttpResponse::Ok().finish())
}
