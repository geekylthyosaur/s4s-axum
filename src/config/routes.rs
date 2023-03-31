use axum::{routing::get, Router};

use crate::{
    routes::{index, user},
    storage::DbPool,
};

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(index))
        .route("/me", get(user::me))
}
