use axum::{
    async_trait,
    extract::{rejection::FormRejection, Form, FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Request},
    Extension, RequestPartsExt, TypedHeader,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::{
    auth::jwt::Claims,
    error::{ApiError, Error},
    models::user::User,
    storage::{user, DbPool},
};

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| Error::from(err))?;
        let Extension(pool) = parts
            .extract::<Extension<DbPool>>()
            .await
            .map_err(|err| Error::from(err))?;
        let claims = Claims::verify(bearer.token())?;
        let user = user::get_by_id(&pool, claims.sub())
            .await
            .map_err(|e| Error::from(e))?
            .ok_or_else(|| Error::WrongCredentials)?;
        Ok(user)
    }
}

pub struct Validated<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Validated<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(form) = Form::<T>::from_request(req, state)
            .await
            .map_err(|e| Error::from(e))?;
        form.validate().map_err(|e| Error::from(e))?;
        Ok(Validated(form))
    }
}
