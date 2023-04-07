use std::ops::Range;

use fake::{
    faker::{
        internet::raw::{Password, SafeEmail, Username},
        name::raw::{FirstName, LastName},
    },
    locales::EN,
    Fake,
};
use serde_json::{json, Value};

use super::TestApp;

impl TestApp {
    pub fn fake_username() -> String {
        Username(EN).fake::<String>()
    }

    pub fn fake_first_name() -> String {
        FirstName(EN).fake::<String>()
    }

    pub fn fake_last_name() -> String {
        LastName(EN).fake::<String>()
    }

    pub fn fake_email() -> String {
        SafeEmail(EN).fake::<String>()
    }

    pub fn fake_password() -> String {
        Password(EN, 8..64).fake::<String>()
    }

    pub fn fake_number(rng: Range<isize>) -> isize {
        rng.fake()
    }

    pub fn fake_text() -> String {
        (0..512).fake()
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

    pub fn fake_edit_form_json() -> Value {
        let username = Self::fake_username();
        let first_name = Self::fake_first_name();
        let last_name = Self::fake_last_name();
        let age = Self::fake_number(0..128);
        let about = Self::fake_text();
        json!({
            "username": username,
            "first_name": first_name,
            "last_name": last_name,
            "age": age,
            "about": about,
        })
    }

    pub fn fake_edit_email_form_json() -> Value {
        let email = Self::fake_email();
        json!({
            "email": email,
        })
    }

    pub fn fake_edit_password_form_json() -> Value {
        let pwd = Self::fake_password();
        json!({
            "password": pwd,
            "repeat_password": pwd,
        })
    }
}
