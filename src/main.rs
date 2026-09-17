// This is the entry point of the application. It ties everything together.

mod config;
mod dto;
mod entity;
mod error;
mod handler;
mod repository;
mod service;
mod routes;
mod states;


use jsonwebtoken::EncodingKey;
use tokio::net::TcpListener;
use std::env;
use dotenvy::dotenv;
use config::db::establish_connection;
use routes::routes::create_router;
use states::appstate::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database Url");
    let pool = establish_connection(&db_url).await.unwrap();

    let secret = env::var("JWT_SECRET").expect("Jwt Secret");
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    let app_state = AppState{
        pool,
        encoding_key
    };

    let app = create_router(app_state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()

}

