use anyhow::Ok;
use tokio::net::TcpListener;
use axum::{Router, routing::get};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use anyhow::Result;


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn establish_connection(db_url: &str) -> Result<PgPool>{
    let pool = PgPoolOptions::new()
                               .max_connections(5)
                               .acquire_timeout(Duration::from_secs(3))
                               .idle_timeout(Duration::from_secs(10))
                               .connect(db_url)
                               .await?;
    Ok(pool)
}

async fn hello_world() -> &'static str{
    "Hello World"
}