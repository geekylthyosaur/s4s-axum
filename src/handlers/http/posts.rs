use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};

use sqlx::SqlitePool;

use crate::{
    handlers::db::posts, 
    models::post::Post
};

pub async fn get_posts(pool: web::Data<SqlitePool>) -> HttpResponse {
    match posts::get_posts(pool.get_ref()).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => e.error_response(),
    }
}

pub async fn get_post(pool: web::Data<SqlitePool>, id: web::Path<u32>) -> HttpResponse {
    match posts::get_post(pool.get_ref(), *id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => e.error_response(),
    }
}

pub async fn create_post(pool: web::Data<SqlitePool>, req: web::Json<Post>) -> HttpResponse {
    match posts::create_post(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn edit_post(pool: web::Data<SqlitePool>, req: web::Json<Post>, id: web::Path<u32>) -> HttpResponse {
    match posts::edit_post(pool.get_ref(), *id, req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_post(pool: web::Data<SqlitePool>, id: web::Path<u32>) -> HttpResponse {
    match posts::delete_post(pool.get_ref(), *id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
