mod config;
mod routes;
mod handlers;
mod models;
mod db;
pub mod middleware;
pub mod services;

use axum::{Router, middleware as axum_mw, Extension};
use tower_http::{
    cors::{CorsLayer, AllowOrigin},
    compression::CompressionLayer,
    trace::TraceLayer,
    limit::RequestBodyLimitLayer,
};
use axum::http::{HeaderValue, Method};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use std::sync::Arc;
use services::email::{EmailService, SmtpEmailService};
use middleware::rate_limit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env();
    let pool = db::init_pool(&cfg.database_url).await?;

    // Email service — production SMTP
    let email_service: Arc<dyn EmailService> = Arc::new(SmtpEmailService {
        from: cfg.notify_email.clone(),
        smtp_host: cfg.smtp_host.clone(),
        smtp_user: cfg.smtp_user.clone(),
        smtp_pass: cfg.smtp_pass.clone(),
    });

    // CORS — restrict to known origins (OWASP: CORS misconfiguration)
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:8045,http://127.0.0.1:8045".into());
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|o| o.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .max_age(std::time::Duration::from_secs(3600));

    // Rate limiter from config
    let rpm: u32 = std::env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);
    let limiter = rate_limit::create_limiter(rpm);

    let app = Router::new()
        .nest("/api/v1", routes::api_router(pool.clone()))
        .layer(Extension(email_service))
        .layer(Extension(limiter))
        .layer(axum_mw::from_fn(rate_limit::rate_limit))
        // Security headers on every response (OWASP)
        .layer(axum_mw::from_fn(middleware::security_headers::security_headers))
        // Limit request body to 1 MB (OWASP: Denial of Service prevention)
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    tracing::info!("🚀 Alashore API → http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
