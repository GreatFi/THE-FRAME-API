use sqlx::types::Uuid;
use crate::dto::auth::Role;
use crate::entity::auth::{User, RefreshToken};
use chrono::{DateTime, Utc};
use crate::states::appstate::AppState;
use crate::error::error::AppError;

pub async fn create_user(app_state: &AppState, name:&str, email:&str, hashed_password:&str) -> Result<User, AppError>{

    let new_user = sqlx::query_as!(User, r#"INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password, role as "role: Role" , created_at, is_verified"#, name, email, hashed_password).fetch_one(&app_state.pool).await.map_err(|e| match e {
    sqlx::Error::Database(ref db) if db.is_unique_violation() => AppError::Conflict,
    other => AppError::DatabaseError(other)})?;

    Ok(new_user)
}


pub async fn get_users(app_state: &AppState) -> Result<Vec<User>, AppError>{
    let users = sqlx::query_as!(User, r#"SELECT id, name, email, password, role as "role: Role", created_at, is_verified FROM users"#).fetch_all(&app_state.pool).await?;
    Ok(users)
}

pub async fn create_ref_token(app_state: &AppState, user_id: Uuid, token_hash:&str, expires_at: DateTime<Utc>) -> Result<RefreshToken, AppError>{


    let ref_token = sqlx::query_as!(
        RefreshToken, 
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, created_at, expires_at", 
        user_id, 
        token_hash,
        expires_at
    ).fetch_one(&app_state.pool).await?;

    Ok(ref_token)
}


pub async fn retrieve_user(app_state: &AppState, email:&str) -> Result<Option<User>, AppError>{
    let user = sqlx::query_as!(User, 
        r#"SELECT id, name, email, password, role as "role: Role", created_at, is_verified FROM users WHERE email=$1"#, email).fetch_optional(&app_state.pool).await?;
    
    Ok(user)
}
pub async fn retrieve_user_by_id(app_state: &AppState, id:Uuid) -> Result<User, AppError>{
    let user = sqlx::query_as!(User, 
        r#"SELECT id, name, email, password, role as "role: Role", created_at, is_verified FROM users WHERE id=$1"#, id).fetch_one(&app_state.pool).await?;
    
    Ok(user)
}

// checking the user role in the db
pub async fn get_user_role(app_state: &AppState, user_id: Uuid) -> Result<Role, AppError>{
    let role = sqlx::query_scalar!( 
        r#"SELECT role as "role: Role" FROM users WHERE id=$1"#, 
        user_id).fetch_one(&app_state.pool).await?;
    Ok(role)
}

pub async fn verify_user(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<bool, AppError>{
    let result = sqlx::query!(r#"UPDATE users
        SET is_verified = true
        WHERE id = $1
          AND is_verified = false"#, user_id).execute(&mut **tx).await?;
    
    Ok(result.rows_affected() == 1)
}

