use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use crate::dto::auth::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub created_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken{
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>
}
