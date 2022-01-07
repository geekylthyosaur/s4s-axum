use actix_web::web::{self, JsonConfig};
use sqlx::PgPool;

use crate::{ 
    handlers::http::{credentials, posts, users, scores}, 
    error::Error 
};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
/*
    GET /posts - get all posts
    POST /posts - create post
    GET /posts/{id} - get one post
    PATCH /posts/{id} - edit one post
    DELETE /posts/{id} - delete one post
    GET /posts/{id}/score - get post`s score
    PATCH /posts/{id}/score/inc - increase post`s score
    PATCH /posts/{id}/score/dec - decrease post`s score

    GET /users - get all users
    POST /users - create user
    GET /users/{id} - get one user
    PATCH /users/{id} - edit one user
    DELETE /users/{id} - delete one user
    GET /users/{id}/credentials - get one user credentials
    PATCH /users/{id}/credentials - edit one user credentials
    PATCH /users/{id}/credentials/toggle - toggle email confirm one user credentials
 */

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
                web::scope("/{id}")
                .service(
                    web::resource("")
                    .route(web::get().to(posts::get_post))
                    .route(web::patch().to(posts::edit_post))
                    .route(web::delete().to(posts::delete_post))
                )
                .service(
                    web::scope("/score")
                    .service(
                        web::resource("")
                        .route(web::get().to(scores::get_score))
                    )
                    .service(
                        web::resource("/inc")
                        .route(web::patch().to(scores::increase_score))
                    )
                    .service(
                        web::resource("/dec")
                        .route(web::patch().to(scores::decrease_score))
                    )
                )
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
                web::scope("/{id}")
                .service(
                web::resource("")
                        .route(web::get().to(users::get_user))
                        .route(web::patch().to(users::edit_user))
                        .route(web::delete().to(users::delete_user))
                )
                .service(
                    web::scope("/credentials")
                    .service(
                        web::resource("")
                        .route(web::get().to(credentials::get_credentials))
                        .route(web::patch().to(credentials::edit_credentials))
                    )
                    .service(
                        web::resource("/toggle")
                        .route(web::patch().to(credentials::toggle_email_confirm))
                    )
                )
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
    PgPool::connect(&dotenv::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable.")).await
}
