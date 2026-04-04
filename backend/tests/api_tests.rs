use axum::http::{Request, StatusCode};
use axum::{Router, Extension};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;
use std::sync::Arc;

use backend::db;
use backend::routes;
use backend::services::email::{EmailService, MockEmailService};
use backend::middleware::rate_limit;

/// Build a test app with in-memory SQLite, mock email, and generous rate limit.
async fn test_app() -> Router {
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let email_svc: Arc<dyn EmailService> = Arc::new(MockEmailService::default());
    let limiter = rate_limit::create_limiter(10_000); // generous for tests
    Router::new()
        .nest("/api/v1", routes::api_router(pool))
        .layer(Extension(email_svc))
        .layer(Extension(limiter))
}

/// Build a test app returning both Router and a handle to the mock email service.
async fn test_app_with_email() -> (Router, Arc<MockEmailService>) {
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let mock = Arc::new(MockEmailService::default());
    let email_svc: Arc<dyn EmailService> = mock.clone();
    let limiter = rate_limit::create_limiter(10_000);
    let app = Router::new()
        .nest("/api/v1", routes::api_router(pool))
        .layer(Extension(email_svc))
        .layer(Extension(limiter));
    (app, mock)
}

/// Helper: send a GET request and return (status, parsed JSON body).
async fn get_json(app: Router, uri: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .uri(uri)
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap();
    (status, json)
}

/// Helper: send a POST request with a JSON body and return (status, parsed JSON body).
async fn post_json(app: Router, uri: &str, body: &Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap();
    (status, json)
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_returns_200_with_correct_json() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["status"], "ok");
    assert_eq!(json["data"]["service"], "alashore-api");
    assert!(json["data"]["version"].is_string());
}

// ---------------------------------------------------------------------------
// Products — list
// ---------------------------------------------------------------------------

#[tokio::test]
async fn list_products_returns_all_six() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/products").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);

    let products = json["data"].as_array().expect("data should be an array");
    assert_eq!(products.len(), 6, "catalog should contain exactly 6 products");

    // Verify each product has the required fields
    for p in products {
        assert!(p["id"].is_string(), "product missing id");
        assert!(p["name"].is_string(), "product missing name");
        assert!(p["category"].is_string(), "product missing category");
        assert!(p["hs_code"].is_string(), "product missing hs_code");
    }
}

// ---------------------------------------------------------------------------
// Products — get by slug
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_product_vannamei_shrimp() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/products/vannamei-shrimp").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["id"], "vannamei-shrimp");
    assert_eq!(json["data"]["name"], "Whiteleg Shrimp");
    assert_eq!(json["data"]["scientific_name"], "Litopenaeus vannamei");
    assert_eq!(json["data"]["category"], "shrimp");
    assert_eq!(json["data"]["featured"], true);
    assert_eq!(json["data"]["hs_code"], "0306.17");
    assert_eq!(json["data"]["min_order_kg"], 20_000);

    // Verify certifications
    let certs = json["data"]["certifications"]
        .as_array()
        .expect("certifications array");
    assert!(certs.iter().any(|c| c == "BAP 4-Star"));
    assert!(certs.iter().any(|c| c == "EU Approved"));

    // Verify specifications are present
    let specs = json["data"]["specifications"]
        .as_array()
        .expect("specifications array");
    assert_eq!(specs.len(), 2);
}

#[tokio::test]
async fn get_product_nonexistent_returns_null_data() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/products/nonexistent").await;

    // The handler wraps the result in ApiResponse::ok(None), so success is true but data is null
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    assert!(json["data"].is_null(), "data should be null for unknown product");
}

// ---------------------------------------------------------------------------
// Audit trail
// ---------------------------------------------------------------------------

#[tokio::test]
async fn audit_log_table_exists() {
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_log")
        .fetch_one(&pool)
        .await
        .expect("audit_log table should exist");
    assert_eq!(count.0, 0, "audit_log should start empty");
}

#[tokio::test]
async fn get_audit_log_returns_empty_initially() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/admin/audit").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    let entries = json["data"].as_array().expect("data should be array");
    assert!(entries.is_empty(), "audit log should be empty initially");
}

#[tokio::test]
async fn inquiry_creates_audit_log_entry() {
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let email_svc: Arc<dyn EmailService> = Arc::new(MockEmailService::default());
    let limiter = rate_limit::create_limiter(10_000);
    let make_app = || {
        Router::new()
            .nest("/api/v1", routes::api_router(pool.clone()))
            .layer(Extension(email_svc.clone()))
            .layer(Extension(limiter.clone()))
    };

    // Submit an inquiry
    let body = serde_json::json!({
        "company_name": "Audit Test Co",
        "contact_name": "Bob",
        "email": "bob@audit.com",
        "country": "UK",
        "product_ids": ["squid"],
        "message": "Testing audit",
        "preferred_contact": "email"
    });
    let (status, _) = post_json(make_app(), "/api/v1/inquiry", &body).await;
    assert_eq!(status, StatusCode::OK);

    // Check audit log has an entry
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_log")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(count.0 >= 1, "audit log should have at least 1 entry after inquiry");

    // Check via API
    let (status2, json) = get_json(make_app(), "/api/v1/admin/audit").await;
    assert_eq!(status2, StatusCode::OK);
    let entries = json["data"].as_array().expect("data array");
    assert!(!entries.is_empty());
    assert_eq!(entries[0]["endpoint"], "/api/v1/inquiry");
    assert_eq!(entries[0]["method"], "POST");
    assert_eq!(entries[0]["status_code"], 200);
}

// ---------------------------------------------------------------------------
// Rate limiting
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rate_limit_returns_429_when_exceeded() {
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let email_svc: Arc<dyn EmailService> = Arc::new(MockEmailService::default());
    // Allow only 1 request per minute
    let limiter = rate_limit::create_limiter(1);
    let make_app = || {
        Router::new()
            .nest("/api/v1", routes::api_router(pool.clone()))
            .layer(axum::middleware::from_fn(rate_limit::rate_limit))
            .layer(Extension(limiter.clone()))
            .layer(Extension(email_svc.clone()))
    };

    // First request should succeed
    let req1 = Request::builder()
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();
    let resp1 = make_app().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);

    // Second request should be rate-limited (429)
    let req2 = Request::builder()
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();
    let resp2 = make_app().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::TOO_MANY_REQUESTS);
}

// ---------------------------------------------------------------------------
// Products — DB persistence: verify data survives across queries
// ---------------------------------------------------------------------------

#[tokio::test]
async fn products_are_seeded_in_database() {
    // Verify products come from DB by querying the pool directly
    let pool = db::init_pool("sqlite::memory:").await.expect("in-memory db");
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await
        .expect("products table should exist and be seeded");
    assert_eq!(count.0, 6, "should have 6 seeded products in DB");
}

#[tokio::test]
async fn product_from_db_has_specifications_and_markets() {
    let app = test_app().await;
    let (status, json) = get_json(app, "/api/v1/products/squid").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["data"]["id"], "squid");

    // Verify nested JSON fields stored/retrieved correctly
    let specs = json["data"]["specifications"].as_array().expect("specs array");
    assert!(!specs.is_empty(), "squid should have specs");

    let markets = json["data"]["markets"].as_array().expect("markets array");
    assert!(markets.len() >= 3, "squid should have 3+ markets");

    let certs = json["data"]["certifications"].as_array().expect("certs array");
    assert!(certs.iter().any(|c| c == "HACCP"), "squid should have HACCP cert");
}

#[tokio::test]
async fn product_db_returns_correct_types() {
    let app = test_app().await;
    let (_, json) = get_json(app, "/api/v1/products/dried-shrimp").await;

    let product = &json["data"];
    assert_eq!(product["category"], "dried");
    assert_eq!(product["featured"], false);
    assert_eq!(product["min_order_kg"], 2000);
    assert!(product["scientific_name"].is_null(), "dried shrimp has no scientific name");
}

// ---------------------------------------------------------------------------
// Inquiry — valid submission
// ---------------------------------------------------------------------------

#[tokio::test]
async fn submit_inquiry_valid() {
    let app = test_app().await;
    let body = serde_json::json!({
        "company_name": "Oceanic Foods Ltd",
        "contact_name": "Jane Doe",
        "email": "jane@oceanicfoods.com",
        "phone": "+1-555-0100",
        "country": "United States",
        "product_ids": ["vannamei-shrimp", "squid"],
        "volume_mt_per_year": 500.0,
        "message": "Interested in quarterly supply of shrimp and squid.",
        "preferred_contact": "email"
    });

    let (status, json) = post_json(app, "/api/v1/inquiry", &body).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    // data contains the UUID of the created inquiry
    assert!(
        json["data"].is_string(),
        "data should be the inquiry UUID string"
    );
    let uuid_str = json["data"].as_str().unwrap();
    assert_eq!(uuid_str.len(), 36, "UUID should be 36 chars");
}

// ---------------------------------------------------------------------------
// Inquiry — verify notification email is sent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn submit_inquiry_sends_notification_email() {
    let (app, mock) = test_app_with_email().await;
    let body = serde_json::json!({
        "company_name": "Test Corp",
        "contact_name": "Alice",
        "email": "alice@testcorp.com",
        "country": "Germany",
        "product_ids": ["squid"],
        "message": "Need 100 MT of squid",
        "preferred_contact": "email"
    });

    let (status, json) = post_json(app, "/api/v1/inquiry", &body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);

    // Give the spawned email task a moment to complete
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1, "one notification email should be sent");
    assert!(sent[0].subject.contains("Test Corp"), "subject should contain company name");
    assert!(sent[0].body.contains("squid"), "body should mention products");
}

// ---------------------------------------------------------------------------
// Inquiry — invalid email
// ---------------------------------------------------------------------------

#[tokio::test]
async fn submit_inquiry_invalid_email() {
    let app = test_app().await;
    let body = serde_json::json!({
        "company_name": "Test Co",
        "contact_name": "Bob",
        "email": "not-an-email",
        "country": "India",
        "product_ids": ["shrimp"],
        "message": "Hello",
        "preferred_contact": "email"
    });

    let (status, json) = post_json(app, "/api/v1/inquiry", &body).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], false);
    assert!(json["error"].is_string());
    assert!(
        json["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("email"),
        "error should mention email"
    );
}

// ---------------------------------------------------------------------------
// Inquiry — missing required fields (deserialization failure)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn submit_inquiry_missing_fields_returns_error() {
    let app = test_app().await;
    // Missing company_name, contact_name, country, product_ids, message
    let body = serde_json::json!({
        "email": "jane@example.com"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/inquiry")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();

    // Axum returns 422 Unprocessable Entity when JSON deserialization fails
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ---------------------------------------------------------------------------
// Newsletter — valid subscription
// ---------------------------------------------------------------------------

#[tokio::test]
async fn newsletter_subscribe_valid() {
    let app = test_app().await;
    let body = serde_json::json!({
        "email": "subscriber@example.com",
        "name": "Alice"
    });

    let (status, json) = post_json(app, "/api/v1/newsletter", &body).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], true);
    assert_eq!(json["data"], "Subscribed");
}

// ---------------------------------------------------------------------------
// Newsletter — invalid email
// ---------------------------------------------------------------------------

#[tokio::test]
async fn newsletter_subscribe_invalid_email() {
    let app = test_app().await;
    let body = serde_json::json!({
        "email": "bad-email"
    });

    let (status, json) = post_json(app, "/api/v1/newsletter", &body).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], false);
    assert!(
        json["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("email"),
        "error should mention email"
    );
}

// ---------------------------------------------------------------------------
// Newsletter — duplicate email is idempotent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn newsletter_duplicate_email_is_idempotent() {
    // Both requests need the same pool so the second insert hits the same DB.
    let pool = db::init_pool("sqlite::memory:")
        .await
        .expect("in-memory db");
    let email_svc: Arc<dyn EmailService> = Arc::new(MockEmailService::default());
    let limiter = rate_limit::create_limiter(10_000);
    let make_app = || {
        Router::new()
            .nest("/api/v1", routes::api_router(pool.clone()))
            .layer(Extension(email_svc.clone()))
            .layer(Extension(limiter.clone()))
    };

    let body = serde_json::json!({
        "email": "dupe@example.com",
        "name": "First"
    });

    // First subscription
    let (s1, j1) = post_json(make_app(), "/api/v1/newsletter", &body).await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(j1["success"], true);

    // Second subscription with same email — should succeed (INSERT OR IGNORE)
    let (s2, j2) = post_json(make_app(), "/api/v1/newsletter", &body).await;
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(j2["success"], true);

    // Verify only one row exists
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE email = ?")
        .bind("dupe@example.com")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1, "duplicate email should result in only one row");
}
