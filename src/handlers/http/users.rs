use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};
use sqlx::PgPool;

use crate::{
    handlers::db::users, 
    models::{
        user::{UserToCreate, UserToUpdate}, 
        json_response::JsonResponse
    },
};

pub async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    match users::get_users(pool.get_ref()).await {
        Ok(users) => 
            HttpResponse::Ok().json(
                JsonResponse::new(Some(users.into_iter().map(|p| p.to_value()).collect()), 
                None)),
        Err(e) => e.error_response(),
    }
}

pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match users::get_user(pool.get_ref(), *id).await {
        Ok(user) => 
            HttpResponse::Ok().json(JsonResponse::new(Some(user.to_value()), None)),
        Err(e) => e.error_response(),
    }
}

pub async fn create_user(pool: web::Data<PgPool>, req: web::Json<UserToCreate>) -> HttpResponse {
    match users::create_user(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn edit_user(pool: web::Data<PgPool>, req: web::Json<UserToUpdate>, id: web::Path<i32>) -> HttpResponse {
    match users::edit_user(pool.get_ref(), *id, req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match users::delete_user(pool.get_ref(), *id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
