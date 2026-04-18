use axum::{extract::State, Json};
use serde::Serialize;
use shared::ApiResponse;
use sqlx::SqlitePool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuditEntry {
    pub id: i64,
    pub timestamp: String,
    pub method: String,
    pub endpoint: String,
    pub status_code: i32,
    pub ip: String,
    pub user_agent: String,
}

pub async fn list_audit_log(State(pool): State<SqlitePool>) -> Json<ApiResponse<Vec<AuditEntry>>> {
    let entries = sqlx::query_as::<_, AuditEntry>(
        "SELECT id, timestamp, method, endpoint, status_code, ip, user_agent
         FROM audit_log ORDER BY timestamp DESC LIMIT 100",
    )
    .fetch_all(&pool)
    .await;

    match entries {
        Ok(entries) => Json(ApiResponse::ok(entries)),
        Err(e) => {
            tracing::error!("Failed to fetch audit log: {e}");
            Json(ApiResponse::err("Failed to load audit log"))
        }
    }
}

/// Insert an audit log entry. Called from the inquiry/newsletter handlers after processing.
pub async fn log_request(pool: &SqlitePool, method: &str, endpoint: &str, status_code: u16) {
    let now = chrono::Utc::now().to_rfc3339();
    if let Err(e) = sqlx::query(
        "INSERT INTO audit_log (timestamp, method, endpoint, status_code) VALUES (?, ?, ?, ?)",
    )
    .bind(&now)
    .bind(method)
    .bind(endpoint)
    .bind(status_code as i32)
    .execute(pool)
    .await
    {
        tracing::warn!("Failed to write audit log: {e}");
    }
}
