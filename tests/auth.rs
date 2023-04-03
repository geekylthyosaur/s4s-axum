mod common;

use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Method, Request, StatusCode},
};
use common::TestResult;
use serde_json::Value;

use crate::common::{DbPool, TestApp};

#[sqlx::test]
fn signup(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/signup")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(
            &TestApp::fake_signup_form_json(),
        )?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = TestApp::access_token_json_schema();

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;

    assert!(schema.validate(&body).is_ok());

    Ok(())
}

#[sqlx::test]
fn login(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);

    let signup_form = TestApp::fake_signup_form_json();
    let login_form = TestApp::fake_login_form_json(&signup_form);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/signup")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&signup_form)?))?;

    let _ = app.oneshot(request).await?;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/login")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&login_form)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = TestApp::access_token_json_schema();

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;

    assert!(schema.validate(&body).is_ok());

    Ok(())
}
