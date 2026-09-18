use crate::error::error::AppError;
use axum::{
    extract::{State},
    Json,
};
use crate::entity::genre::Genre;
use crate::repository::genre::get_genres;
use crate::states::appstate::AppState;
pub async fn get_genres_handler(State(app_state): State<AppState>) -> Result<Json<Vec<Genre>>, AppError>{
    let genres = get_genres(&app_state).await?;
    Ok(Json(genres))
}


