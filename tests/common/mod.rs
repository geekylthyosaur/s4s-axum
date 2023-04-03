use std::error::Error;

use axum::{body::Body, http::Request, response::Response, Router};
use once_cell::sync::Lazy;
use s4s::{config::routes::routes, telemetry::Telemetry};
use sqlx::PgPool;
use tower::ServiceExt;

pub type DbPool = PgPool;

static TRACING: Lazy<()> = Lazy::new(|| {
    Telemetry::initialize();
});

pub struct TestApp {
    app: Router,
}

impl TestApp {
    pub fn spawn(pool: DbPool) -> Self {
        Lazy::force(&TRACING);

        let app = routes().with_state(pool);

        Self { app }
    }

    pub async fn oneshot(&mut self, request: Request<Body>) -> Result<Response, Box<dyn Error>> {
        Ok(self.app.ready().await?.oneshot(request).await?)
    }
}
