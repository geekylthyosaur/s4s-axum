#![warn(clippy::pedantic)]

use std::{net::SocketAddr, time::Duration};

use s4s::{
    config::{routes::routes, Config},
    telemetry::Telemetry,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    Telemetry::initialize();

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

    let addr = SocketAddr::from(config.app.address().expect("Failed to parse address!"));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
