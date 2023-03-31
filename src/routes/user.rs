use axum::Json;

use crate::{error::ApiResult, models::user::User};

pub async fn me(user: ApiResult<User>) -> ApiResult<Json<User>> {
    user.map(|u| Json(u))
}
