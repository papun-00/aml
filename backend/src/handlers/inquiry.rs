use axum::{Json, extract::State, Extension};
use sqlx::SqlitePool;
use shared::{ApiResponse, InquiryRequest};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use crate::middleware::validate::{validate_inquiry, sanitize, truncate, MAX_MESSAGE_LEN, MAX_PHONE_LEN};
use crate::services::email::{EmailService, build_inquiry_email};
use crate::handlers::audit;

pub async fn submit_inquiry(
    State(pool): State<SqlitePool>,
    Extension(email_svc): Extension<Arc<dyn EmailService>>,
    Json(req): Json<InquiryRequest>,
) -> Json<ApiResponse<String>> {
    // Validate inputs (OWASP: Input Validation)
    if let Err(msg) = validate_inquiry(
        &req.company_name,
        &req.contact_name,
        &req.email,
        &req.country,
        &req.product_ids,
        &req.message,
    ) {
        return Json(ApiResponse::err(msg));
    }

    // Sanitize all user-supplied text (OWASP: Output Encoding / Stored XSS prevention)
    let id   = Uuid::new_v4().to_string();
    let now  = Utc::now().to_rfc3339();
    let company = sanitize(&req.company_name);
    let name = sanitize(&req.contact_name);
    let email = req.email.trim().to_lowercase();
    let phone = sanitize(&truncate(&req.phone.unwrap_or_default(), MAX_PHONE_LEN));
    let country = sanitize(&req.country);
    let message = sanitize(&truncate(&req.message, MAX_MESSAGE_LEN));
    let products_json = serde_json::to_string(&req.product_ids).unwrap_or_default();
    let volume = req.volume_mt_per_year;

    let result = sqlx::query(
        "INSERT INTO inquiries
         (id, company_name, contact_name, email, phone, country,
          product_ids, volume_mt, message, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&company)
    .bind(&name)
    .bind(&email)
    .bind(&phone)
    .bind(&country)
    .bind(&products_json)
    .bind(volume)
    .bind(&message)
    .bind(&now)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!("New inquiry #{id} from {name} <{email}>");

            // Audit log
            audit::log_request(&pool, "POST", "/api/v1/inquiry", 200).await;

            // Send notification email (fire-and-forget — don't block the response)
            let notify_email = std::env::var("NOTIFY_EMAIL")
                .unwrap_or_else(|_| "export@alashoremarine.com".into());
            let email_msg = build_inquiry_email(
                &notify_email, &id, &company, &name, &email,
                &country, &req.product_ids, volume, &message,
            );
            let svc = email_svc.clone();
            tokio::spawn(async move {
                if let Err(e) = svc.send(&email_msg).await {
                    tracing::warn!("Failed to send inquiry notification email: {e}");
                }
            });

            Json(ApiResponse::ok(id))
        }
        Err(e) => {
            tracing::error!("Inquiry save failed: {e}");
            Json(ApiResponse::err("Failed to submit. Please try again."))
        }
    }
}
