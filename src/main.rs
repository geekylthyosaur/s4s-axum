use axum::{routing::get, Router};
use std::net::SocketAddr;

use s4s::telemetry::Telemetry;

#[tokio::main]
async fn main() {
    Telemetry::initialize();

    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}
