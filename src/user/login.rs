use super::auth::validate_credentials;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    email: String,
    password: Secret<String>,
}

pub async fn login(pool: Data<PgPool>, form: Json<LoginForm>) -> HttpResponse {
    match validate_credentials(&pool, form.email.clone(), form.password.clone()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
