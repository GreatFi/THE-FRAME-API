use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::dto::product::Category;  

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre{
    pub id: i32,
    pub name: String,
    pub category: Category,
    pub created_at: Option<DateTime<Utc>>
}