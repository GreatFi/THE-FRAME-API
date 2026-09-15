use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRequest{
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenRequest{
    pub user_id: String,
    pub token: String,
    pub expires_at: Option<DateTime<Utc>>
}

