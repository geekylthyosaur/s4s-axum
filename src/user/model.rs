use argon2::{password_hash::PasswordHasher, Argon2};
use secrecy::{ExposeSecret, Secret};
use validator::{Validate, ValidationError};

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
    pub credentials: Credentials,
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
    pub fn new(username: String, about: Option<String>, credentials: Credentials) -> Self {
        NewUser {
            username,
            about,
            credentials,
        }
    }
}

#[derive(Validate, Clone)]
pub struct Credentials {
    #[validate(email(message = "Isn't valid email."))]
    pub email: String,
    pub password: Secret<String>,
}

impl Credentials {
    pub fn new(email: String, password: Secret<String>) -> Self {
        Self { email, password }
    }

    pub fn calc_pwd_hash(
        self,
        salt: &String,
    ) -> Result<Secret<String>, argon2::password_hash::Error> {
        Ok(Secret::new(
            Argon2::default()
                .hash_password(self.password.expose_secret().as_bytes(), &salt)?
                .to_string(),
        ))
    }
}
