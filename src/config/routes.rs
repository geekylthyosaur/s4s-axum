use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{auth, index, user},
    storage::DbPool,
};

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(index))
        .route("/me", get(user::me))
        .route("signup", post(auth::signup))
        .route("login", post(auth::login))
}
