use crate::error::error::AppError;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Duration, Utc};
use rand::{TryRng};
use rand::rngs::SysRng;
use base64::engine::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Sha256, Digest};
use anyhow::Result;
use sqlx::types::Uuid;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header};


use crate::states::appstate::AppState;
use crate::entity::auth::{User};
use crate::repository::auth::{create_ref_token, create_user, retrieve_user};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: Uuid,
   exp: DateTime<Utc>
}

pub async fn register(app_state: &AppState, name:&str, email:&str, password:&str) -> Result<(User, String, String), AppError> {
    let hashed_password = hash_password(password)?;
    let new_user = create_user(app_state, name, email, &hashed_password).await?;
    let ref_token = generate_refresh_token(app_state, new_user.id).await?;
    let access_token = generate_access_token(app_state, new_user.id, Utc::now() + Duration::minutes(30)).await?;
    Ok((new_user, ref_token, access_token))
}

pub async fn login(app_state: &AppState, email:&str, password: &str) -> Result<(User, String, String), AppError>{
    
    let user = retrieve_user(app_state, email).await?;

    let logged_user = match user {
        Some(user) => {
            let verified_password = argon2::Argon2::default().verify_password(password.as_bytes(), user.password.as_str());
            match verified_password{
                Ok(()) => {
                    let ref_token = generate_refresh_token(app_state, user.id).await?;
                    let access_token = generate_access_token(app_state, user.id, Utc::now() + Duration::minutes(30)).await?;
                    Ok((user, ref_token, access_token))
                }
                Err(_) => Err(AppError::Unauthorized)
            }
        }
        None => Err(AppError::Unauthorized)
    };
    logged_user
}


// password hashing
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let argon2 = Argon2::default();

    let hashed = argon2
        .hash_password(password.as_bytes())? 
        .to_string();

    Ok(hashed)
}

// generate a refresh token, hash it, and store it in the database
pub async fn generate_refresh_token(app_state: &AppState, user_id: Uuid) -> Result<String, AppError> {
    let mut bytes = [0u8; 32];
    SysRng.try_fill_bytes(&mut bytes)?; 

    let token = URL_SAFE_NO_PAD.encode(bytes);

    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());

    let hash = hasher.finalize();
    let token_hash = hex::encode(hash);

    create_ref_token(
        app_state,
        user_id,
        &token_hash,
        Utc::now() + Duration::days(7),
    )
    .await?;

    Ok(token)
}

// generate an access token and return it
pub async fn generate_access_token(app_state: &AppState, user_id: Uuid, expiry:DateTime<Utc>) -> Result<String, AppError>{
    
    let my_claims = Claims{
        sub: user_id,
        exp: expiry
    };
    let token = encode(&Header::default(), &my_claims, &app_state.encoding_key)?;

    Ok(token)
}
