use argon2::{Argon2, PasswordHasher};
use rand::{TryRng};
use rand::rngs::SysRng;
use base64::engine::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Sha256, Digest};
use sqlx::postgres::PgPool;
use anyhow::Result;
use sqlx::types::Uuid;
use crate::entity::auth::{User, RefreshToken};
use chrono::{Duration, Utc};
use hex;

pub async fn create_user(pool: &PgPool, name:&str, email:&str, password:&str) -> Result<User>{
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes())?.to_string(); 

    let new_user = sqlx::query_as!(User, "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password, created_at", name, email, hashed_password).fetch_one(pool).await?;

    Ok(new_user)
}


pub async fn get_users(pool: &PgPool) -> Result<Vec<User>>{
    let users = sqlx::query_as!(User, "SELECT id, name, email, password, created_at FROM users").fetch_all(pool).await?;
    Ok(users)
}

pub async fn create_ref_token(pool: &PgPool, user_id: Uuid) -> Result<String>{
    let mut bytes = [0u8; 32];
    SysRng.try_fill_bytes(&mut bytes)?;

    let token = URL_SAFE_NO_PAD.encode(bytes);

    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let hash = hasher.finalize();

    let token_hash = hex::encode(hash);

    sqlx::query_as!(
        RefreshToken, 
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, created_at, expires_at", 
        user_id, 
        token_hash,
        Utc::now() + Duration::days(7)
    ).fetch_one(pool).await?;

    Ok(token)
}

