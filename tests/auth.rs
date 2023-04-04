pub mod common;

use hyper::StatusCode;

use crate::common::{Assert, DbPool, TestApp, TestResult};

#[sqlx::test]
fn signup(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request =
        TestApp::post_request_with_json_body("/auth/signup", TestApp::json_to_body(signup_form)?)?;
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

    let request =
        TestApp::post_request_with_json_body("/auth/signup", TestApp::json_to_body(signup_form)?)?;
    let _ = app.oneshot(request).await?;

    let request =
        TestApp::post_request_with_json_body("/auth/login", TestApp::json_to_body(login_form)?)?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::access_token_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}
