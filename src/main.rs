#![warn(clippy::pedantic)]

use std::net::SocketAddr;

use s4s::{config::routes::routes, telemetry::Telemetry};

#[tokio::main]
async fn main() {
    Telemetry::initialize();

    let app = routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
