// This module specifies the error types and handling for the application.

use rand::rngs::SysError;
use thiserror::Error;
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Forbidden")]
    Forbidden,
    #[error("Conflict")]
    Conflict,
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad request")]
    BadRequest,
    #[error("Internal server error")]
    InvalidOtpData,
    #[error("Internal server error")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("Internal server error")]
    EmailBuildError(#[from] lettre::error::Error),
    #[error("Internal server error")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("Validation failed")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Internal server error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Internal server error")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Internal server error")]
    Argon2Error(#[from] argon2::password_hash::Error),
    #[error("Internal server error")]
    SysRngError(#[from] SysError),
    #[error("Internal server error")]
    ConfigError(#[from] std::env::VarError),
    #[error("Internal server error")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Internal server error")]
    AesKeyError(#[from] aes_gcm::aes::cipher::InvalidLength),
    #[error("Internal server error")]
    AesGcmError(#[from] aes_gcm::Error),
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        tracing::error!("request failed: {:?}", self);
        let body = self.to_string();

        let status = match self {
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonWebTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Argon2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SysRngError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Base64Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AesKeyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AesGcmError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidOtpData => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AddressError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EmailBuildError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SmtpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}

