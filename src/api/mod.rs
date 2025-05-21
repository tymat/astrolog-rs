pub mod routes;
pub mod handlers;
pub mod models;

use axum::Router;

pub fn create_router() -> Router {
    routes::create_router()
} 