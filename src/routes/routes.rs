// this module defines the routes for the API.

use axum::{Router, routing::get};

use crate::handler::product::{
    health_check_handler,
    get_products_handler,
    create_product_handler,
    get_product_by_id_handler,
    update_product_by_id_handler,
    delete_product_handler,
};
use crate::handler::genre::get_genres_handler;
use crate::handler::auth::{create_user_handler, get_users_handler};
use crate::states::appstate::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check_handler))
        .route("/auth/register", get(get_users_handler).post(create_user_handler))
        .route("/genres", get(get_genres_handler))
        .route("/products", get(get_products_handler).post(create_product_handler))
        .route("/products/{id}", get(get_product_by_id_handler).put(update_product_by_id_handler).delete(delete_product_handler))
        .with_state(app_state)
}
 