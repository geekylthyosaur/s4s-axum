use actix_web::web::{self, JsonConfig};

use sqlx::{SqlitePool};

use crate::{ handlers::http::posts, error::Error };

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(
                web::resource("/")
                    .route(web::post().to(posts::create_post))
                    .route(web::get().to(posts::get_posts))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(posts::get_post))
                    .route(web::patch().to(posts::edit_post))
                    .route(web::delete().to(posts::delete_post))
            )
    );
}

pub fn configure_json() -> JsonConfig {
    JsonConfig::default().error_handler(|err, req| {
        Error::json_error_handler(err, req)
    })
}

pub async fn configure_db() -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect("sqlite://target/blog.db").await
}
