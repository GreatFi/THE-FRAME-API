use sqlx::postgres::PgPool;
use jsonwebtoken::{EncodingKey, DecodingKey};
use governor::DefaultKeyedRateLimiter;
use uuid::Uuid;
use std::sync::Arc;
use crate::service::email::Mailer;

#[derive(Clone)]
pub struct AppState{
    pub pool:PgPool,
    pub encoding_key:EncodingKey,
    pub decoding_key: DecodingKey,
    pub mailer: Mailer,
    pub send_limiter: Arc<DefaultKeyedRateLimiter<Uuid>>
}

