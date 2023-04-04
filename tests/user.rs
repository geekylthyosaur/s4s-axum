pub mod common;

use hyper::StatusCode;

use crate::common::{Assert, DbPool, TestApp, TestResult};

#[sqlx::test]
fn get_all(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);

    let request = TestApp::get_request_with_empty_body("/users")?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::users_get_all_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}

#[sqlx::test]
fn get_by_username(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();
    let username = signup_form["username"].as_str().unwrap().to_owned();

    let request =
        TestApp::post_request_with_json_body("/auth/signup", TestApp::json_to_body(signup_form)?)?;
    let _ = app.oneshot(request).await?;

    let request = TestApp::get_request_with_empty_body(&format!("/users/{}", username))?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::users_get_by_username_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}

#[sqlx::test]
fn delete(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request =
        TestApp::post_request_with_json_body("/auth/signup", TestApp::json_to_body(signup_form)?)?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let request = TestApp::delete_request_with_auth_header("/users/me", &token)?;
    let response = app.oneshot(request).await?;

    let status = response.status();

    debug_assert_eq!(status, StatusCode::NO_CONTENT);

    Ok(())
}

#[sqlx::test]
fn me(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request =
        TestApp::post_request_with_json_body("/auth/signup", TestApp::json_to_body(signup_form)?)?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let request = TestApp::get_request_with_auth_header("/users/me", &token)?;
    let response = app.oneshot(request).await?;

    let status = response.status();
    let body = TestApp::body_to_json(response.into_body()).await?;
    let schema = TestApp::users_me_json_schema();

    Assert(status, body)
        .status(StatusCode::OK)
        .json_body_with_schema(schema);

    Ok(())
}
