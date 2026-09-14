// this module is responsible for the database operations related to products in the ecommerce application.

use sqlx::postgres::{PgPool};
use sqlx::types::BigDecimal;
use anyhow::Result;
use crate::entity::product::Product;
use crate::dto::product::Category;

// This function is responsible for the creation of products in the database.
pub async fn create_product(pool: &PgPool, title:&str, author:&str, category:Category, genre_id:i32, price:BigDecimal, image:&str, desc:&str) -> Result<Product>{
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

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>>{
    let products = sqlx::query_as!(Product, 
        r#"SELECT id, title, author, category as "category: Category", genre_id, price, image, description, created_at FROM products"#
    ).fetch_all(pool).await?;

    Ok(products)
}

pub async fn get_product_by_id(pool: &PgPool, id:i32) -> Result<Product> {
    let single_product = sqlx::query_as!(Product,r#"SELECT id, title, author, category as "category: Category", genre_id, price, image, description, created_at FROM products WHERE id = $1"#, id).fetch_one(pool).await?;
    Ok(single_product)
}

pub async fn update_product(id:i32, title: &str, author: &str, category:Category, genre_id:i32, price:BigDecimal, image:&str, desc:&str, pool: &PgPool) -> Result<Product>{
    let updated = sqlx::query_as!(Product, 
        r#"UPDATE products 
        SET title = $1,
            author = $2, 
            category = $3,
            genre_id = $4,
            price=$5,
            image = $6,
            description = $7
            WHERE id = $8
            RETURNING id, title, author, category as "category: Category", genre_id, price, image, description, created_at"#,
            title, author, category as Category, genre_id, price, image, desc, id)
        .fetch_one(pool)
        .await?;
    Ok(updated)
}

pub async fn delete_product(pool: &PgPool, id:i32) -> Result<()>{
    sqlx::query!(r#"DELETE FROM products WHERE id = $1"#, id)
        .execute(pool)
        .await?;
    Ok(())
}
