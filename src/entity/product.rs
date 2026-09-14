// this file represents the product entity/row in the database

use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use serde::{Serialize, Deserialize};
use crate::dto::product::Category;  

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product{
    pub id: i32,
    pub title: String,
    pub author: String,
    pub category: Category,
    pub genre_id: Option<i32>,
    pub price: BigDecimal,
    pub image: String,
    pub description: String,
    pub created_at: Option<DateTime<Utc>>
}