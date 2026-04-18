use axum::Json;
use serde_json::Value;
use shared::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "status": "ok",
        "service": "alashore-api",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
