mod common;

use axum::{body::Body, http::StatusCode};
use common::TestResult;

use crate::common::{DbPool, TestApp};

#[sqlx::test]
fn signup(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestApp::post_request_with_json_body(
        "/auth/signup",
        Body::from(serde_json::to_vec(&signup_form)?),
    )?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = TestApp::access_token_json_schema();
    let body = TestApp::body_to_json(response.into_body()).await?;

    assert!(schema.validate(&body).is_ok());

    Ok(())
}

#[sqlx::test]
fn login(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();
    let login_form = TestApp::fake_login_form_json(&signup_form);

    let request = TestApp::post_request_with_json_body(
        "/auth/signup",
        Body::from(serde_json::to_vec(&signup_form)?),
    )?;
    let _ = app.oneshot(request).await?;

    let request = TestApp::post_request_with_json_body(
        "/auth/login",
        Body::from(serde_json::to_vec(&login_form)?),
    )?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = TestApp::access_token_json_schema();
    let body = TestApp::body_to_json(response.into_body()).await?;

    assert!(schema.validate(&body).is_ok());

    Ok(())
}
