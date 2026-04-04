use axum::Json;
use shared::ApiResponse;
use serde_json::Value;

pub async fn health_check() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "status": "ok",
        "service": "alashore-api",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
