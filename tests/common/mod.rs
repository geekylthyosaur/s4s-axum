use std::error::Error;

use axum::{body::Body, http::Request, response::Response, Router};
use fake::{
    faker::internet::raw::{Password, SafeEmail, Username},
    locales::EN,
    Fake,
};
use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use s4s::{config::routes::routes, telemetry::Telemetry};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

pub type DbPool = PgPool;
pub type TestResult<T> = Result<T, Box<dyn Error>>;

static TRACING: Lazy<()> = Lazy::new(|| {
    Telemetry::initialize();
});

static ACCESS_TOKEN_JSON_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    let schema = json!({
        "type": "object",
        "properties": {
            "access_token": { "type": "string" },
            "token_type": { "type": "string" }
        },
        "required": ["access_token", "token_type"]
    });

    JSONSchema::options().compile(&schema).unwrap()
});

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
}

impl TestApp {
    pub fn fake_username() -> String {
        Username(EN).fake::<String>()
    }

    pub fn fake_email() -> String {
        SafeEmail(EN).fake::<String>()
    }

    pub fn fake_password() -> String {
        Password(EN, 8..64).fake::<String>()
    }

    pub fn fake_signup_form_json() -> Value {
        let pwd = Self::fake_password();
        json!({
            "username": Self::fake_username(),
            "email": Self::fake_email(),
            "password": pwd,
            "repeat_password": pwd,
        })
    }

    pub fn fake_login_form_json(signup_form: &Value) -> Value {
        let username = signup_form.get("username").unwrap();
        let password = signup_form.get("password").unwrap();
        json!({
            "username": username,
            "password": password,
        })
    }
}

impl TestApp {
    pub fn access_token_json_schema() -> &'static JSONSchema {
        &*ACCESS_TOKEN_JSON_SCHEMA
    }
}
