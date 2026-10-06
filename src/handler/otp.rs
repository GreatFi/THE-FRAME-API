use axum::{Json, extract::State};
use crate::service::otp::{send_otp, verify_otp};
use crate::states::appstate::AppState;
use crate::dto::otp::{SendOtpRequest, VerifyOtpRequest};
use crate::dto::auth::AuthResponse;
use crate::error::error::AppError;

pub async fn send_otp_handler(State(app_state): State<AppState>, Json(payload): Json<SendOtpRequest>) -> Result<Json<()>, AppError> {
    let otp = send_otp(&app_state, payload.user_id).await?;

    Ok(Json(otp))
}

pub async fn verify_otp_handler(State(app_state): State<AppState>, Json(payload): Json<VerifyOtpRequest>) -> Result<Json<AuthResponse>, AppError> {
    let (user, refresh_token, access_token) =
        verify_otp(
            &app_state,
            payload.user_id,
            &payload.otp,
        )
        .await?;

    Ok(Json(AuthResponse {
        user,
        refresh_token,
        access_token,
    }))
}