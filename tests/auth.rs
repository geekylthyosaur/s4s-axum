mod common;

use std::error::Error;

use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Method, Request, StatusCode},
};
use jsonschema::is_valid;
use serde_json::{json, Value};

use crate::common::{DbPool, TestApp};

#[sqlx::test]
fn signup(pool: DbPool) -> Result<(), Box<dyn Error>> {
    let mut app = TestApp::spawn(pool);
    let signup_form = json!({
        "username": "geekylthyosaur",
        "email": "geekylthyosaur@gmail.com",
        "password": "pwd",
        "repeat_password": "pwd",
    });
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/signup")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&signup_form)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = json!({
        "type": "object",
        "properties": {
            "access_token": { "type": "string" },
            "token_type": { "type": "string" }
        },
        "required": ["access_token", "token_type"]
    });

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;

    assert!(is_valid(&schema, &body));

    Ok(())
}

#[sqlx::test]
fn login(pool: DbPool) -> Result<(), Box<dyn Error>> {
    let mut app = TestApp::spawn(pool);
    let signup_form = json!({
        "username": "geekylthyosaur",
        "email": "geekylthyosaur@gmail.com",
        "password": "pwd",
        "repeat_password": "pwd",
    });
    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/signup")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&signup_form)?))?;

    let _ = app.oneshot(request).await?;

    let login_form = json!({
        "username": "geekylthyosaur",
        "password": "pwd",
    });

    let request = Request::builder()
        .method(Method::POST)
        .uri("/auth/login")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&login_form)?))?;

    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let schema = json!({
        "type": "object",
        "properties": {
            "access_token": { "type": "string" },
            "token_type": { "type": "string" }
        },
        "required": ["access_token", "token_type"]
    });

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: Value = serde_json::from_slice(&body)?;

    assert!(is_valid(&schema, &body));

    Ok(())
}
