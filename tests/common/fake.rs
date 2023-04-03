use fake::{
    faker::internet::raw::{Password, SafeEmail, Username},
    locales::EN,
    Fake,
};
use serde_json::{json, Value};

use super::TestApp;

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
