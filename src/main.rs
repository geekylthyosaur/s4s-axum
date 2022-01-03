use actix_web::{
    web, 
    App, 
    HttpServer
};

use config::{
    configure_app, 
    configure_db, 
    configure_json
};

mod config;
mod handlers;
mod models;
mod error;

#[actix_web::main]
async fn actix_run() -> std::io::Result<()> {
    let pool = match configure_db().await {
        Ok(p) => web::Data::new(p),
        Err(e) => panic!("{}", e),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(configure_json)
            .configure(configure_app)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    match actix_run() {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    };
}
