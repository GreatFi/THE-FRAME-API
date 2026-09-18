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
    DatabaseError(#[from] sqlx::Error),
    #[error("Internal server error")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Internal server error")]
    Argon2Error(#[from] argon2::password_hash::Error),
    #[error("Internal server error")]
    SysRngError(#[from] SysError),
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {

        let body = self.to_string();

        let status = match self {
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonWebTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Argon2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SysRngError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}

