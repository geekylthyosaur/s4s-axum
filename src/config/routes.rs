use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{auth, index, user},
    storage::DbPool,
};

pub fn routes() -> Router<DbPool> {
    let user_routes = Router::new()
        .route("/", get(user::get_all))
        .route("/me", get(user::me))
        .route(
            "/:username",
            get(user::get_by_username).delete(user::delete),
        );

    let auth_routes = Router::new()
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login));

    Router::new()
        .route("/", get(index))
        .nest("/user", user_routes)
        .nest("/auth", auth_routes)
}
