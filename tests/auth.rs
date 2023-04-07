pub mod common;

use hyper::StatusCode;

use crate::common::{Assert, DbPool, TestApp, TestRequest, TestResult};

#[sqlx::test]
fn signup(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    Assert(response)
        .status(StatusCode::OK)
        .json_schema(TestApp::access_token_json_schema())
        .await;

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

    Assert(response)
        .status(StatusCode::OK)
        .json_schema(TestApp::access_token_json_schema())
        .await;

    Ok(())
}
