use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyOtpRequest {
    pub user_id: Uuid,
    pub otp: String,
}