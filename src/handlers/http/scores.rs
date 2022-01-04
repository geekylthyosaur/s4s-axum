use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    handlers::db::scores, 
    models::{
        score::{ScoreToCreate}, 
        json_response::JsonResponse
    },
};

pub async fn get_score(pool: web::Data<PgPool>, uuid: web::Path<Uuid>) -> HttpResponse {
    match scores::get_score(pool.get_ref(), *uuid).await {
        Ok(score) => 
            HttpResponse::Ok().json(JsonResponse::new(Some(score.to_value()), None)),
        Err(e) => e.error_response(),
    }
}

pub async fn create_score(pool: web::Data<PgPool>, req: web::Json<ScoreToCreate>) -> HttpResponse {
    match scores::create_score(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn increase_score(pool: web::Data<PgPool>, req: web::Path<Uuid>) -> HttpResponse {
    match scores::increase_score(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn decrease_score(pool: web::Data<PgPool>, req: web::Path<Uuid>) -> HttpResponse {
    match scores::decrease_score(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_score(pool: web::Data<PgPool>, uuid: web::Path<Uuid>) -> HttpResponse {
    match scores::delete_score(pool.get_ref(), uuid.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
