use argon2::{Argon2, PasswordHasher};
use sqlx::postgres::PgPool;
use anyhow::Result;
use crate::entity::auth::User;

pub async fn create_user(pool: &PgPool, name:&str, email:&str, password:&str) -> Result<User>{
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes())?.to_string(); 

    let new_user = sqlx::query_as!(User, "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password, created_at", name, email, hashed_password).fetch_one(pool).await?;

    Ok(new_user)
}


pub async fn get_users(pool: &PgPool) -> Result<Vec<User>>{
    let users = sqlx::query_as!(User, "SELECT id, name, email, password, created_at FROM users").fetch_all(pool).await?;
    Ok(users)
}