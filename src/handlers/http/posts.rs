use actix_web::{Error, HttpResponse, web};

use sqlx::SqlitePool;

use crate::{handlers::db::posts, models::post::Post};

pub async fn get_posts(conn: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let posts = posts::get_posts(conn.get_ref()).await.unwrap();
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_post(conn: web::Data<SqlitePool>, id: web::Path<u32>) -> Result<HttpResponse, Error> {
    let post = posts::get_post(conn.get_ref(), *id).await.unwrap();
    Ok(HttpResponse::Ok().json(post))
}

pub async fn create_post(conn: web::Data<SqlitePool>, req: web::Json<Post>) -> Result<HttpResponse, Error> {
    let post = Post { title: req.title.to_owned(), content: req.content.to_owned() };
    posts::create_post(conn.get_ref(), post).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn edit_post(conn: web::Data<SqlitePool>, req: web::Json<Post>, id: web::Path<u32>) -> Result<HttpResponse, Error> {
    let post = Post { title: req.title.to_owned(), content: req.content.to_owned() };
    posts::edit_post(conn.get_ref(), *id, post).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_post(conn: web::Data<SqlitePool>, id: web::Path<u32>) -> Result<HttpResponse, Error> {
    posts::delete_post(conn.get_ref(), *id).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}
