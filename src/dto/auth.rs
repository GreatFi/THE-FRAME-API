use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::entity::auth::User;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRequest{
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenRequest{
    pub user_id: String,
    pub token: String,
    pub expires_at: Option<DateTime<Utc>>
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub refresh_token: String,
    pub access_token: String,
}
