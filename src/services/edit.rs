use crate::{
    auth::password,
    dtos::user::{EditUserEmailForm, EditUserForm, EditUserPasswordForm},
    error::{ApiError, ApiResult, Error},
    models::user::User,
};

pub trait Edit<T> {
    fn with(self, other: T) -> Self;
}

pub trait TryEdit<T> {
    type Error;

    fn try_with(self, other: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Edit<EditUserForm> for User {
    fn with(self, other: EditUserForm) -> Self {
        User {
            username: other.username,
            first_name: other.first_name,
            last_name: other.last_name,
            age: other.age,
            about: other.about,
            ..self
        }
    }
}

impl Edit<EditUserEmailForm> for User {
    fn with(self, other: EditUserEmailForm) -> Self {
        User {
            email: other.email,
            ..self
        }
    }
}

impl TryEdit<EditUserPasswordForm> for User {
    type Error = ApiError;

    fn try_with(self, other: EditUserPasswordForm) -> ApiResult<Self> {
        Ok(User {
            pwd_hash: password::hash(&other.password).map_err(Error::from)?,
            ..self
        })
    }
}
