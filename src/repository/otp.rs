use crate::entity::otp::Otp;
use crate::error::error::AppError;
use crate::states::appstate::AppState;
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub async fn create_otp(app_state: &AppState, user_id: Uuid, code: String, expires_at: DateTime<Utc>) -> Result<Otp, AppError>{
    let otp = sqlx::query_as!(Otp, 
        r#"INSERT INTO otp (user_id, code, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, code, expires_at, used, created_at, attempts"#, 
        user_id, code, expires_at
    ).fetch_one(&app_state.pool).await?;
    Ok(otp)
}

pub async fn get_otp(app_state: &AppState, user_id: Uuid) -> Result<Option<Otp>, AppError> {
    let otp = sqlx::query_as!(
        Otp,
        r#"
        SELECT id, user_id, code, expires_at, used, created_at, attempts
        FROM otp
        WHERE user_id = $1
          AND used = false
          AND expires_at > NOW()
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(&app_state.pool)
    .await?;

    Ok(otp)
}
pub async fn increment_otp_attempts(app_state: &AppState, id: i32) -> Result<(), AppError> {
    sqlx::query!("UPDATE otp SET attempts = attempts + 1 WHERE id = $1", id)
        .execute(&app_state.pool)
        .await?;
    Ok(())
}
pub async fn mark_otp_used(otp_id: i32, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE otp
        SET used = true
        WHERE id = $1
        "#,
        otp_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}