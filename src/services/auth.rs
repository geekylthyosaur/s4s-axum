use tracing::instrument;
use uuid::Uuid;

use crate::{
    auth::password,
    dtos::auth::{LoginForm, SignupForm},
    error::{Error, Result},
    models::user::User,
    storage::{user, DbPool},
};

pub struct Auth;

impl Auth {
    #[instrument(skip(pool))]
    pub async fn signup(pool: &DbPool, form: SignupForm) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let pwd_hash = password::hash(&form.password)?;
        let now = chrono::offset::Utc::now();
        let user = User {
            id,
            username: form.username,
            first_name: None,
            last_name: None,
            email: form.email,
            pwd_hash,
            age: None,
            about: None,
            verified: false,
            created_at: now,
            updated_at: now,
        };
        user::create(pool, user).await?;
        Ok(id)
    }

    #[instrument(skip(pool))]
    pub async fn login(pool: &DbPool, form: LoginForm) -> Result<Uuid> {
        let user = user::get_by_username(pool, form.username).await?;

        if password::verify(&form.password, &user.pwd_hash)? {
            Ok(user.id)
        } else {
            Err(Error::WrongCredentials)
        }
    }
}
