// This module is responsible for handling the product-related endpoints in the ecommerce application.

use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::entity::product::Product;
use crate::dto::product::{ProductRequest};
use crate::repository::product::{create_product, get_products, get_product_by_id, update_product, delete_product};


// health check endpoint to verify that the server is running and responsive
pub async fn health_check_handler() -> &'static str {
    "OK"
}
/*
 This function is responsible for the request handling of the product ceation endpoint. 
 It takes in the request payload, calls the create_product function to insert the product into the database, and returns the created product as a JSON response.
*/

pub async fn create_product_handler(State(pool): State<PgPool>, Json(payload): Json<ProductRequest>) -> Result<Json<Product>, (StatusCode, String)>{
    let product = create_product(&pool, &payload.title, &payload.author, payload.category, payload.genre_id, payload.price, &payload.image, &payload.description).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(product))
}

// functions to retrieve all products from the database
pub async fn get_products_handler(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, (StatusCode, String)>{
    // products here means products retrieved
    let products = get_products(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(products))
}

// functions to retrieve a single product by its ID from the database
pub async fn get_product_by_id_handler(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Product>, (StatusCode, String)>{
    let single_product = get_product_by_id(&pool, id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(single_product))
}


// functions to update/edit a product by its ID in the database
pub async fn update_product_by_id_handler(State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload): Json<ProductRequest>) -> Result<Json<Product>, (StatusCode, String)>{
    let updated_product = update_product(
        id, 
        &payload.title, 
        &payload.author, 
        payload.category, 
        payload.genre_id, 
        payload.price,
        &payload.image, 
        &payload.description,
        &pool
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_product))
}

// functions to delete a product by its ID in the database
pub async fn delete_product_handler(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)>{
    delete_product(&pool, id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}