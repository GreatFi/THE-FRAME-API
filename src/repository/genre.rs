use anyhow::Result;
use crate::entity::genre::Genre;
use crate::dto::product::Category;
use crate::states::appstate::AppState;
use crate::error::error::AppError;
pub async fn get_genres(app_state: &AppState) -> Result<Vec<Genre>, AppError>{
    let genres = sqlx::query_as!(Genre, r#"SELECT id, name, category as "category: Category", created_at  FROM genres"#).fetch_all(&app_state.pool).await?;
    Ok(genres)
}
