// this module defines the routes for the API.

use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put}};

use crate::{handler::product::{
    create_product_handler, delete_product_handler, get_product_by_id_handler, get_products_handler, health_check_handler, update_product_by_id_handler,
}, middleware::{authentication::authentication_middleware, authorization::authorization_middleware}};
use crate::handler::genre::get_genres_handler;
use crate::handler::auth::{create_user_handler, get_users_handler, login_handler};
use crate::states::appstate::AppState;

pub fn create_router(app_state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/", get(health_check_handler))
        .route("/auth/register", get(get_users_handler).post(create_user_handler))
        .route("/auth/login", post(login_handler))
        .route("/genres", get(get_genres_handler))
        .route("/products", get(get_products_handler))
        .route("/products/{id}", get(get_product_by_id_handler));
    
    let protected_routes = Router::new()
        .route("/products", post(create_product_handler))
        .route("/products/{id}", put(update_product_by_id_handler).delete(delete_product_handler)).layer(from_fn_with_state(app_state.clone(), authorization_middleware)).layer(from_fn_with_state(app_state.clone(), authentication_middleware));

    let router = public_routes.merge(protected_routes);

    router.with_state(app_state)
}
 