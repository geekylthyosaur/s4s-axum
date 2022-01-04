use actix_web::{
    HttpResponse, 
    ResponseError, 
    web
};

use sqlx::PgPool;

use crate::{
    handlers::db::posts, 
    models::{
        post::{PostToCreate, PostToUpdate}, 
        json_response::JsonResponse
    },
};

pub async fn get_posts(pool: web::Data<PgPool>) -> HttpResponse {
    match posts::get_posts(pool.get_ref()).await {
        Ok(posts) => 
            HttpResponse::Ok().json(
                JsonResponse::new(Some(posts.into_iter().map(|p| p.to_value()).collect()), 
                None)),
        Err(e) => e.error_response(),
    }
}

pub async fn get_post(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match posts::get_post(pool.get_ref(), *id).await {
        Ok(post) => 
            HttpResponse::Ok().json(JsonResponse::new(Some(post.to_value()), None)),
        Err(e) => e.error_response(),
    }
}

pub async fn create_post(pool: web::Data<PgPool>, req: web::Json<PostToCreate>) -> HttpResponse {
    match posts::create_post(pool.get_ref(), req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn edit_post(pool: web::Data<PgPool>, req: web::Json<PostToUpdate>, id: web::Path<i32>) -> HttpResponse {
    match posts::edit_post(pool.get_ref(), *id, req.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_post(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    match posts::delete_post(pool.get_ref(), *id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
