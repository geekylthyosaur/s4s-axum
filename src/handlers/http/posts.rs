use actix_web::{HttpResponse, web, ResponseError};

use sqlx::SqlitePool;

use crate::{handlers::db::posts, models::post::Post};

pub async fn get_posts(conn: web::Data<SqlitePool>) -> HttpResponse {
    let posts = posts::get_posts(conn.get_ref()).await.unwrap();
    HttpResponse::Ok().json(posts)
}

pub async fn get_post(conn: web::Data<SqlitePool>, id: web::Path<u32>) -> HttpResponse {
    match posts::get_post(conn.get_ref(), *id).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => e.error_response(),
    }
}

pub async fn create_post(conn: web::Data<SqlitePool>, req: web::Json<Post>) -> HttpResponse {
    let post = Post { title: req.title.to_owned(), content: req.content.to_owned() };
    match posts::create_post(conn.get_ref(), post).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn edit_post(conn: web::Data<SqlitePool>, req: web::Json<Post>, id: web::Path<u32>) -> HttpResponse {
    let post = Post { title: req.title.to_owned(), content: req.content.to_owned() };
    match posts::edit_post(conn.get_ref(), *id, post).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_post(conn: web::Data<SqlitePool>, id: web::Path<u32>) -> HttpResponse {
    match posts::delete_post(conn.get_ref(), *id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.error_response(),
    }
}
