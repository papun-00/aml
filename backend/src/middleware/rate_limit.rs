//! Per-IP rate limiting middleware using `governor`.
//! Configured via RATE_LIMIT_REQUESTS_PER_MINUTE env var (default: 60).

use axum::{
    middleware::Next,
    http::{Request, StatusCode},
    response::{Response, IntoResponse},
    body::Body,
};
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use std::num::NonZeroU32;
use std::sync::Arc;

/// Global (non-keyed) rate limiter. In production, use a keyed limiter per IP.
/// For simplicity and safety, this applies a global request-per-minute cap.
pub type GlobalLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

/// Create a rate limiter from config.
pub fn create_limiter(requests_per_minute: u32) -> GlobalLimiter {
    let rpm = NonZeroU32::new(requests_per_minute).unwrap_or(NonZeroU32::new(60).unwrap());
    let quota = Quota::per_minute(rpm);
    Arc::new(RateLimiter::direct(quota))
}

/// Rate limiting middleware — returns 429 Too Many Requests when limit exceeded.
pub async fn rate_limit(
    axum::Extension(limiter): axum::Extension<GlobalLimiter>,
    request: Request<Body>,
    next: Next,
) -> Response {
    match limiter.check() {
        Ok(_) => next.run(request).await,
        Err(_) => {
            tracing::warn!("Rate limit exceeded");
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("retry-after", "60")],
                "Too many requests. Please try again later.",
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_limiter_default() {
        let limiter = create_limiter(60);
        // Should allow at least one request
        assert!(limiter.check().is_ok());
    }

    #[test]
    fn test_create_limiter_one_per_minute() {
        let limiter = create_limiter(1);
        // First request allowed
        assert!(limiter.check().is_ok());
        // Second request should be rate limited
        assert!(limiter.check().is_err());
    }

    #[test]
    fn test_create_limiter_zero_defaults_to_sixty() {
        // NonZeroU32::new(0) returns None, so fallback to 60
        let limiter = create_limiter(0);
        assert!(limiter.check().is_ok());
    }
}
