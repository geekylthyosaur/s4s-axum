use std::fmt;

use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::env::JWT_SECRET, error::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    exp: i64,
}

impl fmt::Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sub)
    }
}

impl Claims {
    pub fn new(id: Uuid) -> Self {
        let iat = chrono::offset::Utc::now();
        let exp = iat + Duration::hours(24);
        Self {
            sub: id,
            exp: exp.timestamp(),
        }
    }

    pub fn sign(self) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )?)
    }

    pub fn verify(token: &str) -> Result<Claims> {
        Ok(jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}
