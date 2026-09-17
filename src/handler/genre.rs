use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::entity::genre::Genre;
use crate::repository::genre::get_genres;
use crate::states::appstate::AppState;
pub async fn get_genres_handler(State(app_state): State<AppState>) -> Result<Json<Vec<Genre>>, (StatusCode, String)>{
    let genres = get_genres(&app_state).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(genres))
}


