use axum::{routing::get, Router};

use crate::routes::index;

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}
