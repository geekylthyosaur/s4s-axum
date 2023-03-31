use uuid::Uuid;

use crate::{dto::{SignupForm, LoginForm}, storage::{DbPool, user}, error::Result, models::user::User, auth::password::hash_password};

pub struct AuthService;

impl AuthService {
    pub async fn signup(pool: &DbPool, form: SignupForm) -> Result<()> {
        let id = Uuid::new_v4();
        let pwd_hash = hash_password(form.password);
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
        Ok(())
    }

    pub async fn login(pool: DbPool, form: LoginForm) -> Result<()> {
        todo!()
    }
}
