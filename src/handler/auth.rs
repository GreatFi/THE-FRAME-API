use crate::error::error::AppError;
use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use crate::{dto::auth::{UserRequest, AuthResponse}, entity::auth::User};
use crate::repository::auth::{get_users};
use crate::states::appstate::AppState;
use crate::service::auth::register;

pub async fn create_user_handler(State(app_state): State<AppState>, Json(payload): Json<UserRequest>) -> Result<Json<AuthResponse>, AppError> {
    let (new_user, refresh_token, access_token) = register(&app_state, &payload.name, &payload.email, &payload.password).await?;
    Ok(Json(AuthResponse { user: new_user, refresh_token, access_token }))
}

pub async fn get_users_handler(State(app_state): State<AppState>, ) -> Result<Json<Vec<User>>, AppError>{
    let users = get_users(&app_state).await?;

    Ok(Json(users))
}