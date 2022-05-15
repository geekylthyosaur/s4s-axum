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

#[derive(Validate)]
pub struct Credentials {
    #[validate(email(message = "Isn't valid email."))]
    pub email: String,
    pub pwd_hash: Secret<String>,
    pub salt: String,
}

impl Credentials {
    pub fn new(
        email: String,
        password: Secret<String>,
        salt: String,
    ) -> Result<Self, argon2::password_hash::Error> {
        let pwd_hash = Secret::new(
            Argon2::default()
                .hash_password(password.expose_secret().as_bytes(), &salt)?
                .to_string(),
        );
        Ok(Self {
            email,
            pwd_hash,
            salt,
        })
    }
}
