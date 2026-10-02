use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Otp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>
}