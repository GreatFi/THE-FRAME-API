use sqlx::postgres::PgPool;
use jsonwebtoken::{EncodingKey, DecodingKey};
use crate::service::email::Mailer;
#[derive(Clone)]
pub struct AppState{
    pub pool:PgPool,
    pub encoding_key:EncodingKey,
    pub decoding_key: DecodingKey,
    pub mailer: Mailer
}
