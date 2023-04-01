use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{ApiResult, Error},
    extractors::{LoggedInUser, LoggedInUserId},
    models::user::User,
    storage::{user, DbPool},
};

pub async fn me(user: ApiResult<LoggedInUser>) -> ApiResult<Json<User>> {
    user.map(|LoggedInUser(u)| Json(u))
}

pub async fn get_all(State(pool): State<DbPool>) -> ApiResult<Json<Vec<User>>> {
    let users = user::get_all(&pool).await.map_err(Error::from)?;

    Ok(Json(users))
}

pub async fn get_by_username(
    State(pool): State<DbPool>,
    Path(username): Path<String>,
) -> ApiResult<Json<User>> {
    let user = user::get_by_username(&pool, username)
        .await
        .map_err(Error::from)?;

    Ok(Json(user))
}

pub async fn delete(
    State(pool): State<DbPool>,
    id: ApiResult<LoggedInUserId>,
) -> ApiResult<StatusCode> {
    let id = id.map(|LoggedInUserId(id)| id)?;
    user::delete(&pool, id).await.map_err(Error::from)?;

    Ok(StatusCode::NO_CONTENT)
}
