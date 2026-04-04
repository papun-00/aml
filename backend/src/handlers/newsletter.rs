use axum::{Json, extract::State};
use sqlx::SqlitePool;
use shared::{ApiResponse, NewsletterRequest};
use chrono::Utc;
use crate::middleware::validate::{validate_newsletter, sanitize, truncate, MAX_NAME_LEN};

pub async fn subscribe(
    State(pool): State<SqlitePool>,
    Json(req): Json<NewsletterRequest>,
) -> Json<ApiResponse<String>> {
    // Validate email (OWASP: Input Validation)
    if let Err(msg) = validate_newsletter(&req.email) {
        return Json(ApiResponse::err(msg));
    }

    let now   = Utc::now().to_rfc3339();
    let email = req.email.trim().to_lowercase();
    let name  = sanitize(&truncate(&req.name.unwrap_or_default(), MAX_NAME_LEN));

    let result = sqlx::query(
        "INSERT OR IGNORE INTO newsletter_subscribers (email, name, subscribed_at) VALUES (?, ?, ?)"
    )
    .bind(&email)
    .bind(&name)
    .bind(&now)
    .execute(&pool)
    .await;

    match result {
        Ok(_)  => Json(ApiResponse::ok("Subscribed".to_string())),
        Err(e) => { tracing::error!("Newsletter error: {e}"); Json(ApiResponse::err("Failed")) }
    }
}
