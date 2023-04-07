pub mod common;

use hyper::StatusCode;

use crate::common::{Assert, DbPool, TestApp, TestResult, TestRequest};

#[sqlx::test]
fn signup(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::access_token_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}

#[sqlx::test]
fn login(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();
    let login_form = TestApp::fake_login_form_json(&signup_form);

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let _ = app.oneshot(request).await?;

    let request = TestRequest::post("/auth/login")
        .with_json(login_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::access_token_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}
