use anyhow::{Result};
use tokio::net::TcpListener;
use axum::{Json, Router, extract::State, http::StatusCode, routing::{get, post}};
use axum::extract::Path;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use sqlx::types::BigDecimal;
use std::env;
use dotenvy::dotenv;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name= "category_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum Category {
    Books, 
    Film, 
    Manga
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProductRequest{
    title: String,
    author: String,
    category: Category,
    genre_id: i32,
    price: BigDecimal,
    image: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Product{
    id: i32,
    title: String,
    author: String,
    category: Category,
    genre_id: Option<i32>,
    price: BigDecimal,
    image: String,
    description: String,
    created_at: Option<DateTime<Utc>>
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database Url");
    let pool = establish_connection(&db_url).await.unwrap();

    let app = Router::new()
        .route("/", get(health_check_handler))
        .route("/products", get(get_products_handler).post(create_product_handler))
        .route("/products/{id}", get(get_productby_id_handler))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()

}
async fn health_check_handler() -> &'static str {
    "OK"
}

async fn establish_connection(db_url: &str) -> Result<PgPool>{
    let pool = PgPoolOptions::new()
                               .max_connections(5)
                               .acquire_timeout(Duration::from_secs(3))
                               .idle_timeout(Duration::from_secs(10))
                               .connect(db_url)
                               .await
                               .expect("Can't connect to the database");
    Ok(pool)
}


/*
 This function is responsible for the request handling of the product ceation endpoint. 
 It takes in the request payload, calls the create_product function to insert the product into the database, and returns the created product as a JSON response.
*/
#[axum::debug_handler]
async fn create_product_handler(State(pool): State<PgPool>, Json(payload): Json<ProductRequest>) -> Result<Json<Product>, (StatusCode, String)>{
    let product = create_product(&pool, &payload.title, &payload.author, payload.category, payload.genre_id, payload.price, &payload.image, &payload.description).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(product))
}

// This function is responsible for the creation of products in the database.
async fn create_product(pool: &PgPool, title:&str, author:&str, category:Category, genre_id:i32, price:BigDecimal, image:&str, desc:&str) -> Result<Product>{
    let product = sqlx::query_as!(Product,r#"
                                     INSERT INTO products (title, author, category, genre_id, price, image, description)
                                     VALUES ($1, $2, $3, $4, $5, $6, $7)
                                     RETURNING id, title, author, category as "category: Category", genre_id, price, image, description, created_at
                                    "#,
                                    title, author, category as Category, genre_id, price, image, desc
                                   ).fetch_one(pool)
                                    .await?;
    Ok(product)
}

// functions to retrieve all products from the database
async fn get_products_handler(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, (StatusCode, String)>{
    // products here means products retrieved
    let products = get_products(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(products))
}
async fn get_products(pool: &PgPool) -> Result<Vec<Product>>{
    let products = sqlx::query_as!(Product, 
        r#"SELECT id, title, author, category as "category: Category", genre_id, price, image, description, created_at FROM products"#
    ).fetch_all(pool).await?;

    Ok(products)
}

// functions to retrieve a single product by its ID from the database
async fn get_productby_id_handler(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Product>, (StatusCode, String)>{
    let single_product = get_product_by_id(&pool, id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(single_product))
}

async fn get_product_by_id(pool: &PgPool, id:i32) -> Result<Product> {
    let single_product = sqlx::query_as!(Product,r#"SELECT id, title, author, category as "category: Category", genre_id, price, image, description, created_at FROM products WHERE id = $1"#, id).fetch_one(pool).await?;
    Ok(single_product)
}