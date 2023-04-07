mod assert;
mod fake;
mod lazy;
mod request;

use std::error::Error;

use axum::{
    body::{Body, HttpBody},
    http::Request,
    response::Response,
    Router,
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use once_cell::sync::Lazy;
use s4s::config::routes::routes;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;

pub use self::assert::Assert;
use self::lazy::TRACING;
pub use self::request::TestRequest;

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

    pub async fn body_to_token<T: HttpBody>(body: T) -> TestResult<String>
    where
        <T as HttpBody>::Error: std::error::Error,
    {
        let body = hyper::body::to_bytes(body).await.unwrap();
        let json: Value = serde_json::from_slice(&body)?;
        let token_type = json["token_type"].as_str().unwrap().to_owned();
        let token = json["access_token"].as_str().unwrap();
        Ok(format!("{} {}", token_type, &token))
    }

    pub fn post_request_with_json_body(uri: &str, body: Body) -> TestResult<Request<Body>> {
        Ok(Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(body)?)
    }

    pub fn get_request_with_empty_body(uri: &str) -> TestResult<Request<Body>> {
        Ok(Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())?)
    }

    pub fn get_request_with_auth_header(uri: &str, header: &str) -> TestResult<Request<Body>> {
        Ok(Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(AUTHORIZATION, header)
            .body(Body::empty())?)
    }

    pub fn delete_request_with_auth_header(uri: &str, header: &str) -> TestResult<Request<Body>> {
        Ok(Request::builder()
            .method(Method::DELETE)
            .uri(uri)
            .header(AUTHORIZATION, header)
            .body(Body::empty())?)
    }
}
