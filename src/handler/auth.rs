use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::{dto::auth::UserRequest, entity::auth::User};
use crate::repository::auth::{create_user, get_users};

pub async fn create_user_handler(State(pool): State<PgPool>, Json(payload): Json<UserRequest>) -> Result<Json<User>, (StatusCode, String)> {
    let new_user = create_user(&pool, &payload.name, &payload.email, &payload.password).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(new_user))
}
pub async fn get_users_handler(State(pool): State<PgPool>, ) -> Result<Json<Vec<User>>, (StatusCode, String)>{
    let users = get_users(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}