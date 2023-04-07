pub mod common;

use hyper::StatusCode;

use crate::common::{Assert, DbPool, TestApp, TestRequest, TestResult};

#[sqlx::test]
fn get_all(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);

    let request = TestRequest::get("/users").build()?;
    let response = app.oneshot(request).await?;

    Assert(response)
        .status(StatusCode::OK)
        .json_schema(TestApp::users_get_all_json_schema())
        .await;

    Ok(())
}

#[sqlx::test]
fn get_by_username(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();
    let username = signup_form["username"].as_str().unwrap().to_owned();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let _ = app.oneshot(request).await?;

    let request = TestRequest::get(format!("/users/{}", username)).build()?;
    let response = app.oneshot(request).await?;

    Assert(response)
        .status(StatusCode::OK)
        .json_schema(TestApp::users_get_by_username_json_schema())
        .await;

    Ok(())
}

#[sqlx::test]
fn delete(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let request = TestRequest::delete("/users/me").with_auth(token).build()?;
    let response = app.oneshot(request).await?;

    Assert(response)
        .status(StatusCode::NO_CONTENT)
        .empty_body()
        .await;

    Ok(())
}

#[sqlx::test]
fn me(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let request = TestRequest::get("/users/me").with_auth(token).build()?;
    let response = app.oneshot(request).await?;

    Assert(response)
        .status(StatusCode::OK)
        .json_schema(TestApp::users_me_json_schema())
        .await;

    Ok(())
}

#[sqlx::test]
fn edit(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let edit_form = TestApp::fake_edit_form_json();

    let request = TestRequest::put("/users/me/edit")
        .with_json(edit_form.clone())
        .with_auth(&token)
        .build()?;

    let response = app.oneshot(request).await?;

    Assert(response).status(StatusCode::NO_CONTENT);

    let request = TestRequest::get("/users/me").with_auth(token).build()?;

    let response = app.oneshot(request).await?;

    Assert(response).json_include(edit_form).await;

    Ok(())
}

#[sqlx::test]
fn edit_email(pool: DbPool) -> TestResult<()> {
    let mut app = TestApp::spawn(pool);
    let signup_form = TestApp::fake_signup_form_json();

    let request = TestRequest::post("/auth/signup")
        .with_json(signup_form)
        .build()?;
    let response = app.oneshot(request).await?;

    let token = TestApp::body_to_token(response.into_body()).await?;

    let edit_form = TestApp::fake_edit_email_form_json();

    let request = TestRequest::put("/users/me/edit/email")
        .with_json(edit_form.clone())
        .with_auth(&token)
        .build()?;

    let response = app.oneshot(request).await?;

    Assert(response).status(StatusCode::NO_CONTENT);

    let request = TestRequest::get("/users/me").with_auth(token).build()?;

    let response = app.oneshot(request).await?;

    Assert(response).json_include(edit_form).await;

    Ok(())
}
