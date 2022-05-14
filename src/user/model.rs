use validator::{Validate, ValidationError};
use secrecy::Secret;

#[derive(Validate)]
pub struct NewUser {
    #[validate(
        length(min = 3, max = 24, message = "Allowed length is 3-24 characters."),
        custom(
            function = "is_ascii_alphabetic_and_lowercase",
            message = "Allowed only a-z characters"
        )
    )]
    pub username: String,
    pub about: Option<String>,
    #[validate(email(message = "Isn't valid email."))]
    pub email: String,
    pub password: Secret<String>,
}

fn is_ascii_alphabetic_and_lowercase(username: &str) -> Result<(), ValidationError> {
    if !username
        .chars()
        .all(|c| char::is_ascii_alphabetic(&c) && char::is_lowercase(c))
    {
        return Err(ValidationError::new("ascii_alphabetic_and_lowercase"));
    }

    Ok(())
}

impl NewUser {
    pub fn new(username: String, about: Option<String>, email: String, password: Secret<String>) -> Self {
        NewUser {
            username,
            about,
            email,
            password,
        }
    }
}

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}
