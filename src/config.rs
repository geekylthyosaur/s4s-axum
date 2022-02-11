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

pub fn configure_db() -> Result<PgPool, sqlx::Error> {
    PgPool::connect_lazy(
        &dotenv::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable."),
    )
}
