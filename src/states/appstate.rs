use sqlx::postgres::PgPool;
use jsonwebtoken::{EncodingKey, DecodingKey};

#[derive(Clone)]
pub struct AppState{
    pub pool:PgPool,
    pub encoding_key:EncodingKey,
    pub decoding_key: DecodingKey
}
