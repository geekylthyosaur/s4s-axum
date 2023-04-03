pub mod fake;
pub mod lazy;

use std::error::Error;

use axum::{body::Body, http::Request, response::Response, Router};
use once_cell::sync::Lazy;
use s4s::config::routes::routes;
use sqlx::PgPool;
use tower::ServiceExt;

use self::lazy::TRACING;

pub type DbPool = PgPool;
pub type TestResult<T> = Result<T, Box<dyn Error>>;

pub struct TestApp {
    app: Router,
}

impl TestApp {
    pub fn spawn(pool: DbPool) -> Self {
        Lazy::force(&TRACING);

        let app = routes().with_state(pool);

        Self { app }
    }

    pub async fn oneshot(&mut self, request: Request<Body>) -> TestResult<Response> {
        Ok(self.app.ready().await?.oneshot(request).await?)
    }
}
