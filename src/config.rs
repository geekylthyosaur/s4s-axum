use actix_web::{
    web::{self, JsonConfig}, 
};

use sqlx::PgPool;

use crate::{ 
    handlers::http::{posts, users}, 
    error::Error 
};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
        .service(
            web::scope("/posts")
            .service(
                web::resource("")
                    .route(web::post().to(posts::create_post))
                    .route(web::get().to(posts::get_posts))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(posts::get_post))
                    .route(web::patch().to(posts::edit_post))
                    .route(web::delete().to(posts::delete_post))
            )
        )
        .service(
            web::scope("/users")
            .service(
                web::resource("")
                    .route(web::post().to(users::create_user))
                    .route(web::get().to(users::get_users))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(users::get_user))
                    .route(web::patch().to(users::edit_user))
                    .route(web::delete().to(users::delete_user))
            )
        )
    );
}

pub fn configure_json() -> JsonConfig {
    JsonConfig::default().error_handler(|err, req| {
        Error::json_error_handler(err, req)
    })
}

pub async fn configure_db() -> Result<PgPool, sqlx::Error> {
    PgPool::connect("postgresql://localhost/blog_db?user=dmytro&password=1111").await
}
