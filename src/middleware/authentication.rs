use axum::extract::FromRequestParts;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use axum::http::header::{AUTHORIZATION, HeaderMap};
use crate::error::error::AppError;
use crate::service::auth::decode_access_token;
use crate::states::appstate::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthenticatedUser{
    pub id: Uuid,
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection>
    {
        let jwt_token = extract_token(&parts.headers)?;
        let decoded_token = decode_access_token(state, jwt_token)?;
        let auth_user = AuthenticatedUser{
            id: decoded_token.sub
        };
        Ok(auth_user)
    }
}

pub fn extract_token(headers: &HeaderMap) -> Result<String, AppError>{
    let token = headers.get(AUTHORIZATION);
    match token {
        Some(header_value) => {
            let bearer = header_value.to_str();
            match bearer{
                Ok(bearer) => {
                    let jwt = bearer.strip_prefix("Bearer ");
                    match jwt{
                        Some(jwt) => {
                            Ok(jwt.to_string())
                        },
                        None => Err(AppError::Unauthorized)
                    }
                }
                Err(_) => Err(AppError::Unauthorized)
            }
        }
        None => Err(AppError::Unauthorized)
    }
}