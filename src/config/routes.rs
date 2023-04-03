use axum::{
    routing::{get, post, put},
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
        .route("/me/edit", put(user::edit))
        .route("/me/edit/email", put(user::edit_email))
        .route("/me/edit/password", put(user::edit_password))
        .route(
            "/:username",
            get(user::get_by_username).delete(user::delete),
        );

    let auth_routes = Router::new()
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login));

    Router::new()
        .route("/", get(index))
        .nest("/users", user_routes)
        .nest("/auth", auth_routes)
}
