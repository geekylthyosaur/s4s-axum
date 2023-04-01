pub mod jwt;
pub mod password;

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{error::{ApiError, Error}, config::env::JWT_SECRET};

use self::jwt::Claims;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::from)?;
        let token_data = decode::<Claims>(bearer.token(), &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default())
            .map_err(Error::from)?;

        Ok(token_data.claims)
    }
}
