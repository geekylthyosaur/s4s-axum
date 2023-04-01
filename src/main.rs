#![warn(clippy::pedantic)]

use std::time::Duration;

use s4s::{
    config::{routes::routes, Config},
    telemetry::Telemetry,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    Telemetry::initialize();

    dotenvy::dotenv().expect("Failed to load .env!");

    let config = Config::new().expect("Failed to read configuration!");

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.storage.connection_string())
        .await
        .expect("Failed to connect to database!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations!");

    let app = routes().with_state(pool);

    axum::Server::bind(&config.app.address().expect("Failed to parse address!"))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
