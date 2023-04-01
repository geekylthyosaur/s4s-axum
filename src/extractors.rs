use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRef, FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Request},
    Json, RequestExt, RequestPartsExt, TypedHeader,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::jwt::Claims,
    error::{ApiError, Error},
    models::user::User,
    storage::{user, DbPool},
};

#[derive(Debug)]
pub struct LoggedInUser(pub User);
#[derive(Debug)]
pub struct LoggedInUserId(pub Uuid);
#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S> FromRequestParts<S> for LoggedInUser
where
    DbPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::from)?;
        let pool = DbPool::from_ref(state);
        let claims = Claims::verify(bearer.token())?;
        let user = user::get_by_id(&pool, claims.sub())
            .await
            .map_err(Error::from)?;
        Ok(LoggedInUser(user))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for LoggedInUserId
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::from)?;
        let claims = Claims::verify(bearer.token())?;
        Ok(LoggedInUserId(claims.sub()))
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: Validate + 'static,
    Json<T>: FromRequest<(), B, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(form) = req.extract::<Json<T>, _>().await.map_err(Error::from)?;
        form.validate().map_err(Error::from)?;
        Ok(Self(form))
    }
}
