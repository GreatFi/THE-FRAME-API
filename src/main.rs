// This is the entry point of the application. It ties everything together.

mod config;
mod dto;
mod entity;
mod error;
mod handler;
mod repository;
mod service;
mod routes;


use tokio::net::TcpListener;
use std::env;
use dotenvy::dotenv;
use config::db::establish_connection;
use routes::routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database Url");
    let pool = establish_connection(&db_url).await.unwrap();

    let app = create_router(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()

}

