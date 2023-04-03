use serde::Deserialize;
use validator::Validate;

use crate::validators::is_lowercase_alphabetic;

#[derive(Debug, Deserialize, Validate)]
pub struct EditUserForm {
    #[validate(length(min = 4, max = 32), custom = "is_lowercase_alphabetic")]
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(range(min = 0, max = 128))]
    pub age: Option<i32>,
    #[validate(length(max = 512))]
    pub about: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EditUserEmailForm {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EditUserPasswordForm {
    #[validate(length(min = 8), must_match(other = "repeat_password"))]
    pub password: String,
    repeat_password: String,
}
