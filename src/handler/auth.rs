use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::{dto::auth::{UserRequest, AuthResponse}, entity::auth::User};
use crate::repository::auth::{create_user, get_users, create_ref_token};

pub async fn create_user_handler(State(pool): State<PgPool>, Json(payload): Json<UserRequest>) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let new_user = create_user(&pool, &payload.name, &payload.email, &payload.password).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let token = create_ref_token(&pool, new_user.id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(AuthResponse { user: new_user, refresh_token: token }))
}

pub async fn get_users_handler(State(pool): State<PgPool>, ) -> Result<Json<Vec<User>>, (StatusCode, String)>{
    let users = get_users(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}