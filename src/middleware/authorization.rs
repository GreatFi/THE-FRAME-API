use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;

use crate::dto::auth::Role;
use crate::error::error::AppError;
use crate::repository::auth::get_user_role;
use crate::middleware::authentication::AuthenticatedUser;
use crate::states::appstate::AppState;

pub async fn authorization(app_state: &AppState,user: &AuthenticatedUser) -> Result<(),AppError> {
    let user_role = get_user_role(app_state, user.id).await?;

    if user_role != Role::Admin{
        return Err(AppError::Forbidden)
    }
    Ok(())
}

pub async fn authorization_middleware(State(state): State<AppState>, request: Request, next:Next) -> Result<Response, AppError>{
    let user = request.extensions().get::<AuthenticatedUser>();

    match user{
        Some(user) =>{
            authorization(&state, user).await?;
            Ok(next.run(request).await)
        }
        None => {
            Err(AppError::Unauthorized)
        }
    }
}