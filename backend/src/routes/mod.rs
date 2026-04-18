use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

pub mod health;
pub mod inquiry;
pub mod newsletter;
pub mod products;

use crate::handlers;

pub fn api_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/products", get(handlers::products::list_products))
        .route("/products/:id", get(handlers::products::get_product))
        .route("/inquiry", post(handlers::inquiry::submit_inquiry))
        .route("/newsletter", post(handlers::newsletter::subscribe))
        .route("/admin/audit", get(handlers::audit::list_audit_log))
        .with_state(pool)
}
