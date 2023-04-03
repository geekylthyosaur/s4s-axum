pub mod fake;
pub mod lazy;

use std::error::Error;

use axum::{
    body::{Body, HttpBody},
    http::Request,
    response::Response,
    Router,
};
use hyper::{header::CONTENT_TYPE, Method};
use once_cell::sync::Lazy;
use s4s::config::routes::routes;
use serde_json::Value;
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

    pub fn json_to_body(json: Value) -> TestResult<Body> {
        Ok(Body::from(serde_json::to_vec(&json)?))
    }

    pub async fn body_to_json<T: HttpBody>(body: T) -> TestResult<Value> {
        let body = unsafe { hyper::body::to_bytes(body).await.unwrap_unchecked() };
        Ok(serde_json::from_slice(&body)?)
    }

    pub fn post_request_with_json_body(uri: &str, body: Body) -> TestResult<Request<Body>> {
        Ok(Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(body)?)
    }
}
