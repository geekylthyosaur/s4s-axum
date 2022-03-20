use actix_web::web;
use sqlx::PgPool;

use crate::health_check::health_check;
use crate::user::signup;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/health_check")
                    .service(web::resource("").route(web::get().to(health_check))),
            )
            .service(
                web::scope("/users")
                    .service(web::resource("").route(web::post().to(signup::signup))),
            ),
    );
}

pub struct DatabaseConfig {
    database_url: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self {
            database_url: format!("postgres://{}:{}@{}:{}/{}",
               dotenv::var("DB_USER").expect("Missing DB_USER environment variable"),
               dotenv::var("DB_PASSWORD").expect("Missing DB_PASSWORD environment variable"),
               dotenv::var("DB_HOST").expect("Missing DB_HOST environment variable"),
               dotenv::var("DB_PORT").expect("Missing DB_PORT environment variable"),
               dotenv::var("DB_NAME").expect("Missing DB_NAME environment variable"))
        }
    }
}

pub fn configure_db() -> Result<PgPool, sqlx::Error> {
    PgPool::connect_lazy(
        &DatabaseConfig::new().database_url
    )
}
