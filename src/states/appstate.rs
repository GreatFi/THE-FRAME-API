use sqlx::postgres::PgPool;
use jsonwebtoken::EncodingKey;

#[derive(Clone)]
pub struct AppState{
    pub pool:PgPool,
    pub encoding_key:EncodingKey
}
