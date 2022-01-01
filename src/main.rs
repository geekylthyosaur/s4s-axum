use actix_web::{web, App, HttpServer};

use config::{configure_app, configure_db};

mod config;
mod handlers;
mod models;

#[actix_web::main]
async fn actix_run() -> std::io::Result<()> {
    let pool = match configure_db().await {
        Ok(p) => web::Data::new(p),
        Err(e) => panic!("{}", e),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(configure_app)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn main() {
    dotenv::dotenv().ok();

    match actix_run() {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    };
}
