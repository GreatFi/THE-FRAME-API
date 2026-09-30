use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::entity::auth::User;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name= "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Staff, 
    Customer
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserRequest{
    #[validate(length(min = 1))]
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 64))]
    pub password: String,
}
fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'') {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_name"))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct LoginRequest{
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 64))]
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
