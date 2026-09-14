/* 
 This module is responsible for establishing a connection to the database using the provided database URL. 
 It uses the `sqlx` crate to create a connection pool for PostgreSQL.
*/ 

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn establish_connection(db_url: &str) -> Result<PgPool>{
    let pool = PgPoolOptions::new()
                               .max_connections(5)
                               .acquire_timeout(Duration::from_secs(3))
                               .idle_timeout(Duration::from_secs(10))
                               .connect(db_url)
                               .await
                               .expect("Can't connect to the database");
    Ok(pool)
}