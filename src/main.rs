// This is the entry point of the application. It ties everything together.

mod config;
mod dto;
mod entity;
mod error;
mod handler;
mod middleware;
mod repository;
mod service;
mod routes;
mod states;

use tracing_subscriber::EnvFilter;
use jsonwebtoken::{DecodingKey, EncodingKey};
use governor::{Quota, RateLimiter};
use std::{num::NonZeroU32, time::Duration};
use std::sync::Arc;
use tokio::net::TcpListener;
use std::env;
use dotenvy::dotenv;
use config::db::establish_connection;
use routes::routes::create_router;
use states::appstate::AppState;
use crate::service::email::Mailer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let db_url = env::var("DATABASE_URL").expect("Database Url");
    let pool = establish_connection(&db_url).await.unwrap();

    let secret = env::var("JWT_SECRET").expect("Jwt Secret");
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let mailer = Mailer::from_env().expect("invalid email config");

    let quota = Quota::with_period(Duration::from_secs(60))
        .expect("period must be non-zero")
        .allow_burst(NonZeroU32::new(3).expect("burst must be non-zero"));
    let send_limiter = Arc::new(RateLimiter::keyed(quota));

    let app_state = AppState{
        pool,
        encoding_key,
        decoding_key,
        mailer,
        send_limiter
    };

    let app = create_router(app_state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()

}

