use anyhow::Result;
use sqlx::types::Uuid;
use crate::entity::auth::{User, RefreshToken};
use chrono::{DateTime, Duration, Utc};
use crate::states::appstate::AppState;

pub async fn create_user(app_state: &AppState, name:&str, email:&str, hashed_password:&str) -> Result<User>{

    let new_user = sqlx::query_as!(User, "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password, created_at", name, email, hashed_password).fetch_one(&app_state.pool).await?;

    Ok(new_user)
}


pub async fn get_users(app_state: &AppState) -> Result<Vec<User>>{
    let users = sqlx::query_as!(User, "SELECT id, name, email, password, created_at FROM users").fetch_all(&app_state.pool).await?;
    Ok(users)
}

pub async fn create_ref_token(app_state: &AppState, user_id: Uuid, token_hash:&str, expires_at: DateTime<Utc>) -> Result<RefreshToken>{


    let ref_token = sqlx::query_as!(
        RefreshToken, 
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, created_at, expires_at", 
        user_id, 
        token_hash,
        expires_at
    ).fetch_one(&app_state.pool).await?;

    Ok(ref_token)
}

