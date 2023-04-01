use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, FromRef, rejection::{FormRejection, JsonRejection}},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Request},
    RequestPartsExt, TypedHeader, RequestExt, Json,
};
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
        Ok(user)
    }
}

pub struct ValidatedJson<T>(pub T);

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
        let Json(form) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(Error::from)?;
        form.validate().map_err(Error::from)?;
        Ok(Self(form))
    }
}
