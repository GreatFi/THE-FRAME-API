use axum::extract::{FromRequestParts, State};
use axum::middleware::Next;
use serde::{Deserialize, Serialize};
use sqlx::decode;
use sqlx::types::Uuid;
use axum::extract::Request;
use axum::response::Response;
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

pub async fn authentication_middleware(State(state): State<AppState>, mut request: Request, next:Next) -> Result<Response, AppError>{
    let token = extract_token(request.headers())?;
    let decoded_token = decode_access_token(&state, token)?;
    let authenticated_user = AuthenticatedUser{
        id: decoded_token.sub
    };
    
    request.extensions_mut().insert(authenticated_user);
    Ok(next.run(request).await)
}