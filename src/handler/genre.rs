use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::entity::genre::Genre;
use crate::repository::genre::get_genres;

pub async fn get_genres_handler(State(pool): State<PgPool>) -> Result<Json<Vec<Genre>>, (StatusCode, String)>{
    let genres = get_genres(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(genres))
}


