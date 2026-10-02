use crate::error::error::AppError;
use axum::{
    extract::{State},
    Json,
};
use validator::Validate;
use crate::{dto::auth::{UserRequest, AuthResponse, LoginRequest}, entity::auth::User};
use crate::repository::auth::{get_users};
use crate::states::appstate::AppState;
use crate::service::auth::{register, login};

pub async fn create_user_handler(State(app_state): State<AppState>, Json(payload): Json<UserRequest>) -> Result<Json<User>, AppError> {
    payload.validate()?;
    let new_user = register(&app_state, &payload.name, &payload.email, &payload.password).await?;
    Ok(Json(new_user))
}

pub async fn login_handler(State(app_state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, AppError>{
    payload.validate()?;
    let (returning_user, refresh_token, access_token) = login(&app_state, &payload.email, &payload.password).await?;
    Ok(Json(AuthResponse{
        user: returning_user,
        refresh_token,
        access_token
    }))
}

pub async fn get_users_handler(State(app_state): State<AppState>, ) -> Result<Json<Vec<User>>, AppError>{
    let users = get_users(&app_state).await?;

    Ok(Json(users))
}