use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};
use sqlx::PgPool;

use crate::{
    handlers::db::credentials, 
    models::{
        credentials::CredentialsToUpdate, 
        json_response::JsonResponse
    },
};

pub async fn get_credentials(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match credentials::get_credentials(pool.get_ref(), id.into_inner()).await {
        Ok(credentials) => 
            HttpResponse::Ok().json(JsonResponse::new(Some(credentials.to_value()), None)),
        Err(e) => e.error_response(),
    }
}

pub async fn edit_credentials(pool: web::Data<PgPool>, id: web::Path<i32>, req: web::Json<CredentialsToUpdate>) -> HttpResponse {
    match credentials::edit_credentials(pool.get_ref(), id.into_inner(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn toggle_email_confirm(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match credentials::toggle_email_confirm(pool.get_ref(), id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
