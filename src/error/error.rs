// This module specifies the error types and handling for the application.

use thiserror::Error;
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode
};
use std::convert::From;

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
    InternalServerError(#[from] sqlx::Error),
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
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}

