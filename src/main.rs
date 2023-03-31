#![warn(clippy::pedantic)]

use std::net::SocketAddr;

use s4s::{
    config::{routes::routes, Config},
    telemetry::Telemetry,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    Telemetry::initialize();

    let config = Config::new();

    let pool = PgPoolOptions::new()
        .connect(&config.storage.connection_string())
        .await
        .expect("Failed to connect to database!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations!");

    let app = routes().with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
