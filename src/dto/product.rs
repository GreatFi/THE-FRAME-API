// this module represents the data transfer objects (DTOs) for the product entity in the application.

use serde::{Serialize, Deserialize};
use sqlx::types::BigDecimal;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name= "category_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Books, 
    Film, 
    Manga
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRequest{
    pub title: String,
    pub author: String,
    pub category: Category,
    pub genre_id: i32,
    pub price: BigDecimal,
    pub image: String,
    pub description: String,
}