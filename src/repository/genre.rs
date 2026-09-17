use sqlx::postgres::{PgPool};
use anyhow::Result;
use crate::entity::genre::Genre;
use crate::dto::product::Category;
use crate::states::appstate::AppState;
pub async fn get_genres(app_state: &AppState) -> Result<Vec<Genre>>{
    let genres = sqlx::query_as!(Genre, r#"SELECT id, name, category as "category: Category", created_at  FROM genres"#).fetch_all(&app_state.pool).await?;
    Ok(genres)
}
