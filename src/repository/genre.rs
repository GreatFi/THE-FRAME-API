use sqlx::postgres::{PgPool};
use anyhow::Result;
use crate::entity::genre::Genre;
use crate::dto::product::Category;

pub async fn get_genres(pool: &PgPool) -> Result<Vec<Genre>>{
    let genres = sqlx::query_as!(Genre, r#"SELECT id, name, category as "category: Category", created_at  FROM genres"#).fetch_all(pool).await?;
    Ok(genres)
}
