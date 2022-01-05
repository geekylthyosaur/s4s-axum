use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};
use sqlx::PgPool;

use crate::{
    handlers::db::scores, 
    models::{
        json_response::JsonResponse
    },
};

pub async fn get_score(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match scores::get_score(pool.get_ref(), id.into_inner()).await {
        Ok(score) => 
            HttpResponse::Ok().json(JsonResponse::new(Some(score.to_value()), None)),
        Err(e) => e.error_response(),
    }
}

pub async fn increase_score(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match scores::increase_score(pool.get_ref(), id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn decrease_score(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match scores::decrease_score(pool.get_ref(), id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
