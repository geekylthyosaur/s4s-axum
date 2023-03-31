use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(length(min = 4, max = 16), custom = "is_lowercase_alphabetic")]
    pub username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(must_match = "repeat_password")]
    pub password: String,
    repeat_password: String,
    #[validate(range(max = 128))]
    age: Option<u8>,
    #[validate(length(max = 512))]
    about: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

fn is_lowercase_alphabetic(s: &str) -> Result<(), ValidationError> {
    s.chars()
        .all(|c| c.is_alphabetic() && c.is_lowercase())
        .then(|| ())
        .ok_or(ValidationError::new(
            "Only lowercase and alphabetic allowed",
        ))
}
